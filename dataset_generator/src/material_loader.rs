use std::path::Path;

use crate::{cel_material::CelMaterial, material_properties_types::MaterialPropertiesRoot};
use bevy::{asset::LoadedAsset, prelude::*, render::texture::ImageType, utils::BoxedFuture};
use bevy_mod_fbx::material_loader::TextureLoader;
use bevy_mod_fbx::utils::fbx_extend::MaterialHandleExt;
use fbxcel_dom::v7400::object::{
    material::MaterialHandle, texture::TextureHandle, TypedObjectHandle,
};

fn find_texture<'a>(material_obj: &MaterialHandle<'a>, name: &str) -> Option<TextureHandle<'a>> {
    material_obj
        .document()
        .objects()
        .filter_map(|obj| match obj.get_typed() {
            TypedObjectHandle::Texture(o) => Some(o),
            _ => None,
        })
        .find(|handle| handle.name().unwrap_or("").contains(name))
}

async fn load_texture<'a, 'w>(
    texture_loader: &mut TextureLoader<'a, 'w>,
    material_obj: &MaterialHandle<'a>,
    tokens: &Vec<&str>,
    sub_name_ind: usize,
    sub_name: &str,
) -> anyhow::Result<Option<Handle<Image>>> {
    let name = format!("{}_{}", tokens[..sub_name_ind].join("_"), sub_name);
    let texture = find_texture(material_obj, &name);

    if let Some(texture) = texture {
        texture_loader
            .get_cached_texture(texture)
            .await
            .map(|h| Some(h))
    } else {
        println!("{} not found in fbx, trying to find in folder", &name);

        let parent = texture_loader.load_context.path().parent().unwrap();
        let name = format!("{name}.png");
        let file = texture_loader
            .load_context
            .asset_io()
            .read_directory(parent)?
            .find(|f| f.ends_with(&name));

        guard! { let Some(file) = file else {
            println!("{} not found in folder as well", name);
            texture_loader.load_context
            .asset_io()
            .read_directory(texture_loader.load_context.path().parent().unwrap())?
            .for_each(|f| println!("{:?}", f));

            return Ok(None)
        } }

        let image_path = Path::new(&file); //parent.join(&file);
        let image = texture_loader
            .load_context
            .read_asset_bytes(image_path)
            .await?;

        let file_ext = Path::new(&file)
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_ascii_lowercase();

        let is_srgb = false; // TODO
        let image = Image::from_buffer(
            &image,
            ImageType::Extension(&file_ext),
            texture_loader.suported_compressed_formats,
            is_srgb,
        )?;

        let handle = texture_loader
            .load_context
            .set_labeled_asset(&name, LoadedAsset::new(image));

        Ok(Some(handle))
    }
}

pub fn load_cel_material<'a, 'w>(
    texture_loader: &'a mut TextureLoader<'a, 'w>,
    material_obj: MaterialHandle<'a>,
) -> BoxedFuture<'a, anyhow::Result<Option<CelMaterial>>> {
    Box::pin(async move {
        println!("start loading {:?}", material_obj.name());

        let diffuse = material_obj.find_texture("DiffuseColor");
        guard! { let Some(diffuse) = diffuse else {
            println!("diffuse property not found");
            return Ok(None)
        } };
        let name = diffuse.name();
        guard! { let Some(name) = name else {
            println!("cannot get name of diffuse property");
            return Ok(None)
        } };

        let is_face = name.contains("Tex_Face");
        let tokens = name.split('_').collect();

        macro_rules! load_optional_map {
            ($name: expr, $ind: expr) => {
                load_texture(texture_loader, &material_obj, &tokens, $ind, $name).await?
            };
        }

        macro_rules! load_map {
            ($name: expr, $ind: expr) => {
                if let Some(map) = load_optional_map!($name, $ind) {
                    map
                } else {
                    println!("{} map not found", $name);
                    return Ok(None);
                }
            };
        }

        println!("{:?}", texture_loader.load_context.path());

        let diffuse = texture_loader.get_cached_texture(diffuse).await?;

        let parent = texture_loader.load_context.path().parent().unwrap();

        let single_model = true; //texture_loader.load_context.asset_io().is_dir(Path::new("Materials"));
        let mat_name = material_obj.name().and_then(|m| m.split(".").next());
        guard! { let Some(mat_name) = mat_name else {
            println!("Cannot parse material name {:?}", material_obj.name());
            return Ok(None)
        } };

        let path = if single_model {
            format!("Materials/{}.json", mat_name)
        } else {
            format!("../Materials/{}.json", mat_name)
        };

        let path = parent.join(path);

        let properties = texture_loader.load_context.read_asset_bytes(path).await?;
        let properties: MaterialPropertiesRoot = serde_json::from_slice(properties.as_slice())?;

        if is_face {
            Ok(Some(CelMaterial::new_face(
                diffuse,
                load_map!("Tex_FaceLightmap", 2),
                load_map!("Tex_Face_Shadow", 1),
                load_map!("Tex_MetalMap", 1),
                load_map!("Body_Shadow_Ramp", 5),
                properties.into(),
            )))
        } else {
            Ok(Some(CelMaterial::new(
                diffuse,
                load_map!("Lightmap", 6),
                load_map!("Shadow_Ramp", 6),
                load_map!("Tex_MetalMap", 1),
                load_optional_map!("Normalmap", 6),
                properties.into(),
            )))
        }
    })
}

pub fn load_cel_material_fallback<'a, 'w>(
    _texture_loader: &'a mut TextureLoader<'a, 'w>,
    material_obj: MaterialHandle<'a>,
) -> BoxedFuture<'a, anyhow::Result<Option<CelMaterial>>> {
    Box::pin(async move {
        println!("using fallback for {:?}", material_obj.name());
        let mut mat = CelMaterial::default();
        mat.is_face = true;
        Ok(Some(mat))
    })
}
