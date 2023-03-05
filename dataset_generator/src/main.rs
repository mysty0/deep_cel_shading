use std::path::{Path, PathBuf};

use bevy::{prelude::*, render::{render_resource::{ShaderType, RenderPipelineDescriptor, SpecializedMeshPipelineError}, texture::ImageType, mesh::MeshVertexBufferLayout}, utils::BoxedFuture, asset::{LoadedAsset, load_internal_asset}, pbr::{MaterialPipeline, MaterialPipelineKey}};
use bevy_mod_fbx::{FbxPlugin, FbxScene, FbxMesh, material_loader::{MaterialLoader, TextureLoader}, FbxMaterialLoaders, FbxLoader, ATTRIBUTE_NORMAL_MAP_UV};
use bevy_editor_pls::{prelude::*};
use bevy::{
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use camera_control::{pan_orbit_camera, spawn_camera};
use fbxcel_dom::v7400::object::{material::MaterialHandle, TypedObjectHandle, texture::TextureHandle};
use self::material_properties_types::MaterialPropertiesRoot;
#[macro_use] extern crate guard;

pub mod material_properties_types;
pub mod camera_control;

#[derive(Component)]
pub struct Spin;

fn load_cel_material<'a, 'w>(
    texture_loader: &'a mut TextureLoader<'a, 'w>, 
    material_obj: MaterialHandle<'a>
) -> BoxedFuture<'a, anyhow::Result<Option<CelMaterial>>> {
    Box::pin(async move { 
        use bevy_mod_fbx::utils::fbx_extend::MaterialHandleExt;

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

        fn find_texture<'a>(material_obj: &MaterialHandle<'a>, name: &str) -> Option<TextureHandle<'a>> {
            material_obj.document().objects()
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
            sub_name: &str
        ) -> anyhow::Result<Option<Handle<Image>>> {
            let name = format!("{}_{}", tokens[..sub_name_ind].join("_"), sub_name);
            let texture = find_texture(material_obj, &name);
            
            if let Some(texture) = texture {
                texture_loader.get_cached_texture(texture).await.map(|h| Some(h))
            } else {
                println!("{} not found in fbx, trying to find in folder", &name);
                
                let parent = texture_loader.load_context.path().parent().unwrap();
                let name = format!("{name}.png");
                let file = texture_loader.load_context
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
                
                let image_path = Path::new(&file);//parent.join(&file);
                let image = texture_loader.load_context.read_asset_bytes(image_path).await?;
                
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
                    return Ok(None) 
                }
            };
        }
        println!("{:?}", texture_loader.load_context.path());

        let diffuse = texture_loader.get_cached_texture(diffuse).await?;

        let parent = texture_loader.load_context.path().parent().unwrap();

        let single_model = true;//texture_loader.load_context.asset_io().is_dir(Path::new("Materials"));
        let mat_name =  material_obj.name().and_then(|m| m.split(".").next());
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
                properties.into()
            )))
        }
    })
}

fn load_cel_material_fallback<'a, 'w>(
    texture_loader: &'a mut TextureLoader<'a, 'w>, 
    material_obj: MaterialHandle<'a>
) -> BoxedFuture<'a, anyhow::Result<Option<CelMaterial>>> {
    Box::pin(async move { 
        println!("using fallback for {:?}", material_obj.name());
        let mut mat = CelMaterial::default();
        mat.is_face = true;
        Ok(Some(mat))
    })
}

const CEL_UTILS_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 31698701027590);

struct CelShaderPlugin;

impl Plugin for CelShaderPlugin {
    fn build(&self, app: &mut App) { 
        load_internal_asset!(app, CEL_UTILS_HANDLE, "shaders/utils.wgsl", Shader::from_wgsl);
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(FbxMaterialLoaders::<CelMaterial>(vec![
            &load_cel_material,
            &load_cel_material_fallback
        ]))
        //.add_plugin(FbxPlugin)
        .init_asset_loader::<FbxLoader<CelMaterial>>()
        .add_asset::<FbxMesh<CelMaterial>>()
        .add_asset::<FbxScene<CelMaterial>>()
        .add_plugin(
            MaterialPlugin::<CelMaterial>::default(),
        )
        .add_plugin(CelShaderPlugin)
        .add_system(pan_orbit_camera)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        //.add_system(spin_cube)
        .add_system(create_cell_materials)
        //.add_system(axis_lines)
        .run();
}

fn axis_lines(
    mut lines: ResMut<DebugLines>,
) {
    lines.line_colored(Vec3::new(0.0, 1.0, 0.0), Vec3::new(4.0, 1.0, 0.0), 0.0, Color::RED);
    lines.line_colored(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 4.0), 0.0, Color::BLUE);
}

fn spin_cube(time: Res<Time>, mut query: Query<&mut Handle<StandardMaterial>, With<Spin>>) {

    //for mut transform in query.iter_mut() {
    //    transform.rotate_local_y(0.3 * time.delta_seconds());
    //    transform.rotate_local_x(0.3 * time.delta_seconds());
    //    transform.rotate_local_z(0.3 * time.delta_seconds());
    //}
}

// fn override_material(
//     mesh_name: &str,
//     light_map: &str,
//     query: &mut Query<(&Name, Entity, &Handle<StandardMaterial>)>,
//     commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
//     face_materials: &mut ResMut<Assets<CelMaterial>>
// ) {
//     let face_ent = query.iter().find(|(name, _, _)| name.as_str() == mesh_name);
//     guard! { let Some((_, face_ent, face_material)) = face_ent else { return } }
//     guard! { let Some(face_material) = materials.get(face_material) else { return } }
//     println!("found {:?}", face_ent);
    
//     let material = CelMaterial {
//         diffuse: face_material.base_color_texture.clone(),
//         light_map: Some(asset_server.load(light_map)),
//     };

//     let material = face_materials.add(material);

//     commands
//         .entity(face_ent)
//         .remove::<Handle<StandardMaterial>>();
//     commands.entity(face_ent).insert(material);
// }

fn create_cell_materials(
   // mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<FbxScene<CelMaterial>>>,
  //  mut assets: ResMut<Assets<FbxScene<CelMaterial>>>,
  //  mut meshes: ResMut<Assets<FbxMesh>>,
 //   mut materials: ResMut<Assets<StandardMaterial>>,
   // mut query: Query<(&Name, Entity, &Handle<StandardMaterial>)>,
    mut query_visibility: Query<(&Name, &mut Visibility)>,
    //mut face_materials: ResMut<Assets<CelMaterial>>,
   // asset_server: Res<AssetServer>,
    //map_img: Res<MyMapImage>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // a texture was just loaded or changed!
                println!("scene loaded");
                // WARNING: this mutable access will cause another
                // AssetEvent (Modified) to be emitted!
                //guard! { let Some(scene) = assets.get_mut(handle) else { 
                //    println!("cant get scene handle");
                //    return 
                //} }
                //let scene = .unwrap();
                // ^ unwrap is OK, because we know it is loaded now

                if let Some((_, mut vis)) = query_visibility.iter_mut().find(|(name, _)| name.as_str() == "EffectMesh") {
                    vis.is_visible = false;
                }

                //println!("meshes {:?}", scene.meshes.iter().map(|m| meshes.get(m.1)?.name.clone()).map(|n| n.unwrap_or(String::new())).collect::<Vec<String>>());
                
                // override_material("Face", "models/fischl/Avatar_Girl_Tex_FaceLightmap.png", &mut query, &mut commands, &asset_server, &mut materials, &mut face_materials);
                // override_material("Face_Eye", "models/fischl/Avatar_Girl_Tex_FaceLightmap.png", &mut query, &mut commands, &asset_server, &mut materials, &mut face_materials);

                // let face_material = scene.materials.get("FbxMaterial@Avatar_Girl_Bow_FischlCostumeHighness_Mat_Face").unwrap();
                // let face_material = materials.get_mut(face_material).unwrap();
                
                // *face_material = FaceMaterial {
                //     color: Color::BLUE,
                //     color_texture: face_material.base_color_texture,
                //     alpha_mode: AlphaMode::Blend,
                // };//StandardMaterial::from(Color::RED);
               
            }
            AssetEvent::Modified { handle } => {
                // an image was modified
            }
            AssetEvent::Removed { handle } => {
                // an image was unloaded
            }
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CelMaterial>>,
) {
    // plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(
            StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }
        ),//materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-0.5, 1.5, 1.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    //     ..default()
    // });
    spawn_camera(&mut commands);

    let model_bundle = SceneBundle {
        scene: asset_server.load("models/Characters/Faruzan/Avatar_Girl_Bow_Faruzan (merge).fbx#Scene"),
        transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
        //scene: asset_server.load("models/cube.fbx#Scene"),//.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
        ..default()
    };
    commands.spawn((
        model_bundle,
        Spin,
    ));
}

#[derive(Debug, Clone, ShaderType)]
struct Direction {
    forward: Vec3,
    right: Vec3
}

impl Default for Direction {
    fn default() -> Self {
        Self { forward: Vec3::new(0.0, 0.0, 1.0), right: Vec3::new(-1.0, 0.0, 0.0) }
    }
}

#[derive(Debug, Clone, ShaderType)]
struct ShadowRamp {
    width: f32,
    day_mult_colors: [Color; 5],
    night_mult_colors: [Color; 5],
    transition_range1: f32,
    transition_range2: f32,
    transition_range3: f32,
    transition_range4: f32,
    transition_range5: f32,
    transition_softness1: f32,
    transition_softness2: f32,
    transition_softness3: f32,
    transition_softness4: f32,
    transition_softness5: f32,
}

impl Default for ShadowRamp {
    fn default() -> Self {
        let default_color = Color::rgba(0.9, 0.7, 0.75, 1.0);
        let default_colors = [
            default_color.clone(), 
            default_color.clone(),
            default_color.clone(), 
            default_color.clone(),
            default_color.clone(),
        ];

        const transition_range: f32 = 0.01;
        const softness: f32 = 0.5;

        Self { 
            width: 1.0,
            day_mult_colors: default_colors.clone(),
            night_mult_colors: default_colors,
            transition_range1: transition_range,
            transition_range2: transition_range,
            transition_range3: transition_range,
            transition_range4: transition_range,
            transition_range5: transition_range,
            transition_softness1: softness,
            transition_softness2: softness,
            transition_softness3: softness,
            transition_softness4: softness,
            transition_softness5: softness,
        }
    }
}

#[derive(Debug, Clone, ShaderType, Default)]
struct MaterialGlobalSpecular {
    shininess1: f32,
    shininess2: f32,
    shininess3: f32,
    shininess4: f32,
    shininess5: f32,
    specular_multi1: f32,
    specular_multi2: f32,
    specular_multi3: f32,
    specular_multi4: f32,
    specular_multi5: f32,
}

#[derive(Debug, Clone, ShaderType, Default)]
pub struct CelMaterialProperties {
    head_direction: Direction,
    shadow_ramp_values: ShadowRamp,
    global_specular: MaterialGlobalSpecular,

    use_materials: Vec4,
    metal_map_light_color: Color,
    metal_map_dark_color: Color,
    metal_map_shadow_multi_color: Color,

    metal_map_sharp_layer_color: Color,
    metal_map_specular_color: Color,

    specular_color: Color,
    hit_color: Color,

    day_night_cycle: f32,

    use_shadow_ramp_texture: f32,
    light_area: f32,
    flip_light_map: f32,
    face_map_softness: f32,
    use_ligth_map_color_ao: f32,
    use_vertex_color_ao: f32,
    
    normal_map_scale: f32,
    use_normal_map: f32,
    use_back_space_uv: f32,

    use_metal_map: f32,
    metal_map_tile_scale: f32,
    metal_map_brightness: f32,
    

    metal_map_shininess: f32,
    metal_map_sharp_layer_offset: f32,
    
    metal_map_specular_atten_in_shadow: f32,
    metal_map_specular_scale: f32,

    use_fresnel: f32,
    hit_color_fresnel_power: f32,
    hit_color_scaler: f32,

    rim_light_type: f32,
    rim_light_intensity: f32,
    rim_light_thickness: f32,
}

impl Into<Color> for material_properties_types::Color {
    fn into(self) -> Color {
        Color::rgba(self.r, self.g, self.b, self.a)
    }
}

impl From<MaterialPropertiesRoot> for CelMaterialProperties {
    fn from(properties: MaterialPropertiesRoot) -> Self {
        let properties = properties.m_saved_properties;
        let floats = properties.m_floats;
        let colors = properties.m_colors;
        CelMaterialProperties {
            head_direction: Default::default(),
            day_night_cycle: 0.0,
            use_shadow_ramp_texture: floats.use_shadow_ramp, 
            use_normal_map: floats.use_bump_map,
            use_back_space_uv: floats.use_back_face_uv2, 
            use_ligth_map_color_ao: floats.use_light_map_color_ao,
            use_vertex_color_ao: floats.use_vertex_color_ao, 
            use_materials: [
                floats.use_material2,
                floats.use_material3,
                floats.use_material4,
                floats.use_material5,
            ].into(), 
            //use_face: floats.use_face_map_new,
            flip_light_map: 0.0, 

            light_area: floats.light_area,
            face_map_softness: floats.face_map_softness, 
            normal_map_scale: floats.bump_scale, 

            shadow_ramp_values: ShadowRamp { 
                width: floats.shadow_ramp_width, 
                day_mult_colors: [
                    colors.first_shadow_mult_color.into(),
                    colors.first_shadow_mult_color2.into(),
                    colors.first_shadow_mult_color3.into(),
                    colors.first_shadow_mult_color4.into(),
                    colors.first_shadow_mult_color5.into(),
                ], 
                night_mult_colors: [
                    colors.cool_shadow_mult_color.into(),
                    colors.cool_shadow_mult_color2.into(),
                    colors.cool_shadow_mult_color3.into(),
                    colors.cool_shadow_mult_color4.into(),
                    colors.cool_shadow_mult_color5.into(),
                ], 
                transition_range1: floats.shadow_transition_range,
                transition_range2: floats.shadow_transition_range2,
                transition_range3: floats.shadow_transition_range3,
                transition_range4: floats.shadow_transition_range4,
                transition_range5: floats.shadow_transition_range5,
            
                transition_softness1: floats.shadow_transition_softness,
                transition_softness2: floats.shadow_transition_softness2,
                transition_softness3: floats.shadow_transition_softness3,
                transition_softness4: floats.shadow_transition_softness4,
                transition_softness5: floats.shadow_transition_softness5,
                
            },

            use_metal_map: floats.metal_material,
            metal_map_tile_scale: floats.mtmap_tile_scale,
            metal_map_brightness: floats.mtmap_brightness,
            metal_map_light_color: colors.mtmap_light_color.into(),
            metal_map_dark_color: colors.mtmap_dark_color.into(),
            metal_map_shadow_multi_color: colors.mtshadow_multi_color.into(),
            metal_map_shininess: floats.mtshininess,
            metal_map_sharp_layer_offset: floats.mtsharp_layer_offset,
            metal_map_sharp_layer_color: colors.mtsharp_layer_color.into(),
            metal_map_specular_color: colors.mtspecular_color.into(),
            metal_map_specular_atten_in_shadow: floats.mtspecular_atten_in_shadow,
            metal_map_specular_scale: floats.mtspecular_scale,
            global_specular: MaterialGlobalSpecular { 
                shininess1: floats.shininess, 
                shininess2: floats.shininess2, 
                shininess3: floats.shininess3, 
                shininess4: floats.shininess4, 
                shininess5: floats.shininess5, 
                specular_multi1: floats.spec_multi, 
                specular_multi2: floats.spec_multi2, 
                specular_multi3: floats.spec_multi3, 
                specular_multi4: floats.spec_multi4, 
                specular_multi5: floats.spec_multi5 
            },
            specular_color: colors.specular_color.into(),
            hit_color: colors.hit_color.into(),
            use_fresnel: 1.0,
            hit_color_fresnel_power: floats.hit_color_fresnel_power,
            hit_color_scaler: floats.hit_color_scaler,
            rim_light_type: 1.0,
            rim_light_intensity: 1.0,
            rim_light_thickness: 1.0,
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3406028"]
#[bind_group_data(CelMaterialKey)]
pub struct CelMaterial {
    is_face: bool,
    #[texture(0)]
    #[sampler(1)]
    diffuse: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    face_light_map: Option<Handle<Image>>,
    #[texture(4)]
    #[sampler(5)]
    light_map: Option<Handle<Image>>,
    #[texture(6)]
    #[sampler(7)]
    shadow_ramp: Option<Handle<Image>>,
    #[texture(8)]
    #[sampler(9)]
    normal_map: Option<Handle<Image>>,
    #[texture(10)]
    #[sampler(11)]
    metal_map: Option<Handle<Image>>,

    #[uniform(12)]
    properties: CelMaterialProperties,
}

impl CelMaterial {
    fn new(
        diffuse: Handle<Image>, 
        light_map: Handle<Image>, 
        shadow_ramp: Handle<Image>, 
        metal_map: Handle<Image>, 
        normal_map: Option<Handle<Image>>, 
        properties: CelMaterialProperties
    ) -> Self {
        Self {
            is_face: false,
            diffuse: Some(diffuse),
            light_map: Some(light_map),
            metal_map: Some(metal_map),
            shadow_ramp: Some(shadow_ramp),
            face_light_map: None,
            normal_map,
            properties,
        }
    }

    fn new_face(
        diffuse: Handle<Image>, 
        face_light_map: Handle<Image>, 
        light_map: Handle<Image>, 
        metal_map: Handle<Image>, 
        shadow_ramp: Handle<Image>, 
        properties: CelMaterialProperties
    ) -> Self {
        Self {
            is_face: true,
            diffuse: Some(diffuse),
            face_light_map: Some(face_light_map),
            metal_map: Some(metal_map),
            light_map: Some(light_map),
            shadow_ramp: Some(shadow_ramp),
            normal_map: None,
            properties
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CelMaterialKey {
    is_face: bool,
}

impl From<&CelMaterial> for CelMaterialKey {
    fn from(material: &CelMaterial) -> Self {
        CelMaterialKey {
            is_face: material.is_face,
        }
    }
}

impl Material for CelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cel_material.wgsl".into()
        //"shaders/test.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/cel_material.wgsl".into()
        //"shaders/test.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if !layout.contains(Mesh::ATTRIBUTE_TANGENT) 
            || !layout.contains(Mesh::ATTRIBUTE_COLOR
        ) {
            if let Some(fragment) = descriptor.fragment.as_mut() {
               fragment.shader_defs.push("EMPTY".into());
            }
            //descriptor.vertex.buffers = vec![layout.get_layout(&[])?];
            return Ok(())
        }

        println!("has color {}", layout.contains(Mesh::ATTRIBUTE_COLOR));

        let mut vertex_attributes = vec![
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_TANGENT.at_shader_location(2),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(3),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(4),
            //Mesh::ATTRIBUTE_UV_0.at_shader_location(5)
        ];

        let mut shader_defs = Vec::new();

        if key.bind_group_data.is_face {
            shader_defs.push("FACE".into());
        } else {
            // if !layout.contains(ATTRIBUTE_NORMAL_MAP_UV) {

            // }
            vertex_attributes.push(ATTRIBUTE_NORMAL_MAP_UV.at_shader_location(5));
            shader_defs.push("VERTEX_NORMAL_MAP_UV".into());
            println!("has normal map uv {}", layout.contains(ATTRIBUTE_NORMAL_MAP_UV));
            
        }

        let vertex_layout = layout.get_layout(&vertex_attributes)?;
        descriptor.vertex.buffers = vec![vertex_layout];

        println!("defs {:?}", &shader_defs);
        descriptor.vertex.shader_defs.extend(shader_defs.clone());
        if let Some(fragment) = descriptor.fragment.as_mut() {
            fragment.shader_defs.extend(shader_defs);
        }
        
        Ok(())
    }
}
