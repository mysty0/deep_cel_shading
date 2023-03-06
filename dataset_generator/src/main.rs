use std::fs::{self, File};
use std::io::{Read, Write};

use bevy::reflect::serde::{ReflectSerializer, UntypedReflectDeserializer};
use bevy::reflect::TypeUuid;
use bevy::tasks::IoTaskPool;
use bevy::{asset::load_internal_asset, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_editor_pls::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_mod_fbx::{FbxLoader, FbxMaterialLoaders, FbxMesh, FbxScene};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use camera_control::{pan_orbit_camera, spawn_camera};
use cel_material::CelMaterial;

use iyes_progress::{ProgressCounter, ProgressPlugin};
use material_loader::{load_cel_material, load_cel_material_fallback};
use serde::Deserialize;

#[macro_use]
extern crate guard;

pub mod camera_control;
pub mod cel_material;
pub mod material_loader;
pub mod material_properties_types;

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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Loading,
    Ready,
    Failed,
}

#[derive(Clone, Resource, Default, Reflect, TypeUuid, serde::Deserialize, serde::Serialize)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b00275901"]
struct Presets {
    camera: Vec<Transform>,
    light: Vec<Transform>,
}

#[derive(AssetCollection, Resource)]
struct AppAssets {
    #[asset(path = "presets.ron")]
    presets: Handle<Presets>,
    #[asset(
        paths(
            "models/Characters/Faruzan/Avatar_Girl_Bow_Faruzan (merge).fbx#Scene",
            //"models/Characters/Aether/Cs_Avatar_Boy_Sword_PlayerBoy #1.fbx#Scene",
            "models/Characters/Collei/NPC_Avatar_Girl_Bow_Collei_Edit.fbx#Scene"
        ),
        collection(typed)
    )]
    models: Vec<Handle<Scene>>,
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
        .add_plugin(RonAssetPlugin::<Presets>::new(&["ron"]))
        //.add_plugin(FbxPlugin)
        .init_asset_loader::<FbxLoader<CelMaterial>>()
        .add_asset::<FbxMesh<CelMaterial>>()
        .add_asset::<FbxScene<CelMaterial>>()
        .add_plugin(MaterialPlugin::<CelMaterial>::default())
        .add_plugin(CelShaderPlugin)
        .add_system(pan_orbit_camera)
        .add_plugin(EguiPlugin)
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Ready)
                .on_failure_continue_to_state(AppState::Failed)
                .with_collection::<AppAssets>(),
        )
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_ui_system))
        .add_plugin(ProgressPlugin::new(AppState::Loading)) //.continue_to(AppState::Ready))
        //.insert_resource(Presets::default())
        //.add_plugin(EditorPlugin)
        //.add_startup_system(setup)
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup))
        .add_system_set(SystemSet::on_update(AppState::Failed).with_system(failed_ui_system))
        .add_system_set(
            SystemSet::on_update(AppState::Ready)
                .with_system(ui_system)
                .with_system(hide_effect_mesh),
        )
        //.add_system(spin_cube)
        //.add_system(axis_lines)
        .run();
}

fn failed_ui_system(mut egui_context: ResMut<EguiContext>) {
    egui::TopBottomPanel::top("top_panel").show(egui_context.ctx_mut(), |ui| {
        ui.label("assets loading failed");
    });
}

fn loading_ui_system(
    mut egui_context: ResMut<EguiContext>,
    progress: Option<Res<ProgressCounter>>,
    mut last_done: Local<u32>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_context.ctx_mut(), |ui| {
        ui.label("Loading...");

        if let Some(progress) = progress.map(|counter| counter.progress()) {
            if progress.done > *last_done {
                *last_done = progress.done;
                println!("Changed progress: {:?}", progress);
            }
            ui.label(format!(
                "Progress: {} {} {}",
                progress.total, progress.done, *last_done
            ));
        }
    });
}

fn ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut presets: ResMut<Presets>,
    camera_query: Query<(&Transform, With<Camera>)>,
    mut light_query: Query<(&mut Transform, With<PointLight>, Without<Camera>)>,
    type_registry: Res<AppTypeRegistry>,
) {
    egui::Window::new("Cameras").show(egui_context.ctx_mut(), |ui| {
        for camera in &presets.camera {
            ui.label(format!("{:?} {:?}", camera.translation, camera.rotation));
        }
        if ui.button("Add").clicked() {
            presets.camera.push(camera_query.single().0.clone());
        }
    });

    let mut light = light_query.single_mut().0;

    egui::Window::new("Move light").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("x: ");
            ui.add(egui::DragValue::new(&mut light.translation.x).speed(0.1));
        });
        ui.horizontal(|ui| {
            ui.label("y: ");
            ui.add(egui::DragValue::new(&mut light.translation.y).speed(0.1));
        });
        ui.horizontal(|ui| {
            ui.label("z: ");
            ui.add(egui::DragValue::new(&mut light.translation.z).speed(0.1));
        });
    });

    egui::Window::new("Lights").show(egui_context.ctx_mut(), |ui| {
        for light in &presets.light {
            ui.label(format!("{:?}", light.translation));
        }
        if ui.button("Add").clicked() {
            presets.light.push(light.clone());
        }
    });

    egui::Window::new("Persistance").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Save").clicked() {
            //let registry = type_registry.read();
            //let serializer = ReflectSerializer::new(presets.as_ref(), &registry);
            let serialized =
                ron::ser::to_string_pretty(presets.as_ref(), ron::ser::PrettyConfig::default())
                    .unwrap();

            IoTaskPool::get()
                .spawn(async move {
                    // Write the scene RON data to file
                    File::create(format!("assets/presets.ron"))
                        .and_then(|mut file| file.write(serialized.as_bytes()))
                        .expect("Error while writing scene to file");
                })
                .detach();
        }
    });
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

fn hide_effect_mesh(
    mut ev_asset: EventReader<AssetEvent<FbxScene<CelMaterial>>>,
    mut query_visibility: Query<(&Name, &mut Visibility)>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle: _ } => {
                println!("scene loaded");

                if let Some((_, mut vis)) = query_visibility
                    .iter_mut()
                    .find(|(name, _)| name.as_str() == "EffectMesh")
                {
                    vis.is_visible = false;
                }
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
    assets: Res<AppAssets>,
    presets: Res<Assets<Presets>>,
) {
    // plane
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::GREEN,
    //         ..default()
    //     }), //materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
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
            shadows_enabled: false,
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

    commands.spawn(SceneBundle {
        scene: assets.models.first().unwrap().clone(), //asset_server
        //.load("models/Characters/Faruzan/Avatar_Girl_Bow_Faruzan (merge).fbx#Scene"),
        transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
        //scene: asset_server.load("models/cube.fbx#Scene"),//.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
        ..default()
    });
    commands.insert_resource(presets.get(&assets.presets).unwrap().clone());
}
