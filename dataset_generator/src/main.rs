use bevy::reflect::TypeUuid;
use bevy::{asset::load_internal_asset, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_mod_fbx::{FbxLoader, FbxMaterialLoaders, FbxMesh, FbxScene};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use camera_control::{pan_orbit_camera, spawn_camera};
use cel_material::CelMaterial;

use material_loader::{load_cel_material, load_cel_material_fallback};

#[macro_use]
extern crate guard;

pub mod camera_control;
pub mod cel_material;
pub mod material_loader;
pub mod material_properties_types;

#[derive(Component)]
pub struct Spin;

const CEL_UTILS_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 31698701027590);

struct CelShaderPlugin;

impl Plugin for CelShaderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            CEL_UTILS_HANDLE,
            "shaders/utils.wgsl",
            Shader::from_wgsl
        );
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
            &load_cel_material_fallback,
        ]))
        //.add_plugin(FbxPlugin)
        .init_asset_loader::<FbxLoader<CelMaterial>>()
        .add_asset::<FbxMesh<CelMaterial>>()
        .add_asset::<FbxScene<CelMaterial>>()
        .add_plugin(MaterialPlugin::<CelMaterial>::default())
        .add_plugin(CelShaderPlugin)
        .add_system(pan_orbit_camera)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        //.add_system(spin_cube)
        .add_system(create_cell_materials)
        //.add_system(axis_lines)
        .run();
}

fn axis_lines(mut lines: ResMut<DebugLines>) {
    lines.line_colored(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(4.0, 1.0, 0.0),
        0.0,
        Color::RED,
    );
    lines.line_colored(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 4.0),
        0.0,
        Color::BLUE,
    );
}

fn spin_cube(_time: Res<Time>, _query: Query<&mut Handle<StandardMaterial>, With<Spin>>) {

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
            AssetEvent::Created { handle: _ } => {
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

                if let Some((_, mut vis)) = query_visibility
                    .iter_mut()
                    .find(|(name, _)| name.as_str() == "EffectMesh")
                {
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
            AssetEvent::Modified { handle: _ } => {
                // an image was modified
            }
            AssetEvent::Removed { handle: _ } => {
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
    _custom_materials: ResMut<Assets<CelMaterial>>,
) {
    // plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        }), //materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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
        scene: asset_server
            .load("models/Characters/Faruzan/Avatar_Girl_Bow_Faruzan (merge).fbx#Scene"),
        transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
        //scene: asset_server.load("models/cube.fbx#Scene"),//.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
        ..default()
    };
    commands.spawn((model_bundle, Spin));
}
