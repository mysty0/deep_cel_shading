use bevy::prelude::*;
use bevy_mod_fbx::{FbxPlugin, FbxScene, FbxMesh};
use bevy_editor_pls::prelude::*;
use bevy::{
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
#[macro_use] extern crate guard;


#[derive(Component)]
pub struct Spin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FbxPlugin)
        .add_plugin(
            MaterialPlugin::<FaceMaterial>::default(),
        )
        //.add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_system(spin_cube)
        .add_system(create_cell_materials)
        .run();
}

fn spin_cube(time: Res<Time>, mut query: Query<&mut Handle<StandardMaterial>, With<Spin>>) {

    //for mut transform in query.iter_mut() {
    //    transform.rotate_local_y(0.3 * time.delta_seconds());
    //    transform.rotate_local_x(0.3 * time.delta_seconds());
    //    transform.rotate_local_z(0.3 * time.delta_seconds());
    //}
}

fn create_cell_materials(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<FbxScene>>,
    mut assets: ResMut<Assets<FbxScene>>,
    mut meshes: ResMut<Assets<FbxMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Name, Entity, &Handle<StandardMaterial>)>,
    mut query_visibility: Query<(&Name, &mut Visibility)>,
    mut face_materials: ResMut<Assets<FaceMaterial>>,
    //map_img: Res<MyMapImage>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // a texture was just loaded or changed!

                // WARNING: this mutable access will cause another
                // AssetEvent (Modified) to be emitted!
                guard! { let Some(scene) = assets.get_mut(handle) else { return } }
                //let scene = .unwrap();
                // ^ unwrap is OK, because we know it is loaded now

                if let Some((_, mut vis)) = query_visibility.iter_mut().find(|(name, _)| name.as_str() == "EffectMesh") {
                    vis.is_visible = false;
                }

                println!("meshes {:?}", scene.meshes.iter().map(|m| meshes.get(m.1)?.name.clone()).map(|n| n.unwrap_or(String::new())).collect::<Vec<String>>());
                
                let face_ent = query.iter().find(|(name, _, _)| name.as_str() == "Face");
                guard! { let Some((_, face_ent, face_material)) = face_ent else { return } }
                guard! { let Some(face_material) = materials.get(face_material) else { return } }
                println!("found {:?}", face_ent);
                
                let material = FaceMaterial {
                    color: face_material.base_color,
                    color_texture: face_material.base_color_texture.clone(),
                    alpha_mode: AlphaMode::Opaque,
                };

                let material = face_materials.add(material);

                commands
                    .entity(face_ent)
                    .remove::<Handle<StandardMaterial>>();
                commands.entity(face_ent).insert(material);

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
    mut custom_materials: ResMut<Assets<FaceMaterial>>,
) {
    // plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: custom_materials.add(
            FaceMaterial {
                color: Color::BLUE,
                color_texture: None,
                alpha_mode: AlphaMode::Opaque,
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-1.0, 1.0, 1.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });

    let model_bundle = SceneBundle {
        scene: asset_server.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
        transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
        //scene: asset_server.load("models/cube.fbx#Scene"),//.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
        ..default()
    };
    commands.spawn((
        model_bundle,
        Spin,
    ));
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct FaceMaterial {
    #[texture(0)]
    #[sampler(1)]
    light_map: Option<Handle<Image>>,
    #[texture(0)]
    #[sampler(1)]
    light_map: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for FaceMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/face_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}