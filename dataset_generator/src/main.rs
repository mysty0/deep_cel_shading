#![feature(result_option_inspect)]

#[macro_use]
extern crate lazy_static;

use std::fs::{self, File};
use std::io::{Read, Write};

use bevy::reflect::serde::{ReflectSerializer, UntypedReflectDeserializer};
use bevy::reflect::TypeUuid;
#[cfg(feature = "screenshot")]
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::tasks::IoTaskPool;
use bevy::utils::HashMap;
use bevy::window::WindowId;
//use bevy::window::PrimaryWindow;
use bevy::{asset::load_internal_asset, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;
#[cfg(feature = "ui")]
use bevy_editor_pls::prelude::*;
#[cfg(feature = "ui")]
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_mod_fbx::{FbxLoader, FbxMaterialLoaders, FbxMesh, FbxScene};
//use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use camera_control::{pan_orbit_camera, spawn_camera};
use cel_material::CelMaterial;

use material_loader::{load_cel_material, load_cel_material_fallback};
use rand::prelude::*;
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

#[derive(Component)]
struct CurrentCharacter;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Loading,
    Ready,
    Failed,
}

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
struct Presets {
    camera: Vec<Transform>,
    light: Vec<Transform>,
}

#[derive(Clone, Default, TypeUuid, serde::Deserialize, serde::Serialize)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b00275901"]
struct PresetsGroups {
    groups: HashMap<String, Presets>,
}

#[derive(Resource)]
struct PresetsResource(Handle<PresetsGroups>);

const PRESETS_FILENAME: &'static str = "presets_new.ron";

const CHARACTERS: &'static [&'static str] = &[
    "models/Characters/Baizhu/NPC_Avatar_Male_Catalyst_Baizhu (merge).fbx#Scene",
    "models/Characters/Barbara/Summer/Avatar_Girl_Catalyst_BarbaraCostumeSummertime.fbx#Scene",
    //broken tangent mapping mode "models/Characters/Childe/Avatar_Male_Bow_Tartaglia_Remote (merge).fbx#Scene",
    "models/Characters/Cyno/NPC_Avatar_Boy_Pole_Cyno.fbx#Scene",
    "models/Characters/Diona/Avatar_Loli_Bow_Diona_Remote (merge).fbx#Scene",
    //"models/Characters/Dvalin/Cs_Monster_Dvalin_S04.fbx#Scene",
    "models/Characters/Fischl/Default/Avatar_Girl_Bow_Fischl_Remote (merge).fbx#Scene",
    //big 
    "models/Characters/Kaveh/NPC_Avatar_Male_Claymore_Kaveh.fbx#Scene",
    //"models/Characters/La Signora/Cs_Monster_LaSignora (merge).fbx#Scene",
    //noface "models/Characters/Naganohara Yoimiya/Avatar_Girl_Bow_Yoimiya_Remote (merge).fbx#Scene",
    "models/Characters/Rosaria/Censored/Avatar_Lady_Pole_Rosaria.fbx#Scene",
    //no face "models/Characters/Scaramouche/NPC_Avatar_Boy_Catalyst_Scaramouche (merge).fbx#Scene",
    //no face "models/Characters/Thoma/NPC_Coop_Avatar_Male_Pole_Tohma_Edit (merge).fbx#Scene",
    //no face "models/Characters/Yun Jin/Avatar_Girl_Pole_Yunjin (merge).fbx#Scene",
    
    "models/Characters/Qiqi/Avatar_Loli_Sword_Qiqi_Remote (merge).fbx#Scene",

    "models/Characters/Kamisato Ayaka/NPC_Avatar_Girl_Sword_Ayaka (merge).fbx#Scene",
    "models/Characters/Mona/Default/NPC_Avatar_Girl_Catalyst_Mona (merge).fbx#Scene",
    "models/Characters/Aether/Cs_Avatar_Boy_Sword_PlayerBoy.fbx#Scene",
    "models/Characters/Candace/NPC_Avatar_Lady_Pole_Candace.fbx#Scene",
    "models/Characters/Amber/Default/Avatar_Girl_Bow_Ambor.fbx#Scene",
    "models/Characters/Albedo/Cs_Avatar_Boy_Sword_Albedo.fbx#Scene",
    "models/Characters/Alhatham/NPC_Avatar_Male_Sword_Alhatham.fbx#Scene",
    "models/Characters/Aloy/Avatar_Girl_Bow_Aloy (merge).fbx#Scene",
    "models/Characters/Arataki Itto/Avatar_Male_Claymore_Itto_Remote (merge).fbx#Scene",
    "models/Characters/Barbara/Default/Avatar_Girl_Catalyst_Barbara.fbx#Scene",
    "models/Characters/Beidou/Avatar_Lady_Claymore_Beidou_Remote (merge).fbx#Scene",
    "models/Characters/Bennett/NPC_Homeworld_Avatar_Boy_Sword_Bennett (merge).fbx#Scene",
    "models/Characters/Chongyun/NPC_Coop_Avatar_Boy_Claymore_Chongyun_Edit (merge).fbx#Scene",
    "models/Characters/Collei/NPC_Avatar_Girl_Bow_Collei_Edit.fbx#Scene",
    "models/Characters/Dainsleif/NPC_Avatar_Male_Sword_Dainslaif (merge).fbx#Scene",
    "models/Characters/Dehya/NPC_Avatar_Lady_Claymore_Dehya.fbx#Scene",
    "models/Characters/Diluc/Default/Cs_Avatar_Male_Claymore_Diluc.fbx#Scene",
    "models/Characters/Diluc/Flamme/Avatar_Male_Claymore_DilucCostumeFlamme.fbx#Scene",
    "models/Characters/Dori/NPC_Avatar_Loli_Claymore_Dori_Edit.fbx#Scene",
    "models/Characters/Eula/NPC_Avatar_Lady_Claymore_Eula (merge).fbx#Scene",
    "models/Characters/Faruzan/Avatar_Girl_Bow_Faruzan (merge).fbx#Scene",
    "models/Characters/Fischl/Highness/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene",
    "models/Characters/Ganyu/Avatar_Girl_Bow_Ganyu (merge).fbx#Scene",
    "models/Characters/Gorou/NPC_Avatar_Boy_Bow_Gorou_Edit (merge).fbx#Scene",
    "models/Characters/Hu Tao/NPC_Avatar_Girl_Pole_Hutao (merge).fbx#Scene",
    //huge mask "models/Characters/IlDotorre/NPC_Avatar_Male_Claymore_IlDotorre.fbx#Scene",
    "models/Characters/Jean/Default/Cs_Avatar_Lady_Sword_Qin.fbx#Scene",
    "models/Characters/Jean/Summer/NPC_Avatar_Lady_Sword_QinCostumeSea_Edit (merge).fbx#Scene",
    "models/Characters/Kaedehara Kazuha/Cs_Avatar_Boy_Sword_Kazuha (merge).fbx#Scene",
    "models/Characters/Kaeya/Cs_Avatar_Male_Sword_Kaeya.fbx#Scene",
    "models/Characters/Kamisato Ayato/Avatar_Male_Sword_Ayato (merge).fbx#Scene",
    "models/Characters/Keqing/Default/NPC_Avatar_Girl_Sword_Keqing (merge).fbx#Scene",
    "models/Characters/Keqing/Feather/NPC_Avatar_Girl_Sword_KeqingCostumeFeather_Edit (merge).fbx#Scene",
    "models/Characters/Klee/Cs_Avatar_Loli_Catalyst_Klee.fbx#Scene",
    "models/Characters/Kujou Sara/NPC_Avatar_Lady_Bow_Sara_Edit (merge).fbx#Scene",
    "models/Characters/Kuki Shinobu/NPC_Homeworld_Avatar_Girl_Sword_Shinobu (merge).fbx#Scene",
    "models/Characters/Lisa/NPC_Avatar_Lady_Catalyst_Lisa_Edit (merge).fbx#Scene",
    "models/Characters/Lumine/Cs_Avatar_Girl_Sword_PlayerGirl.fbx#Scene",
    "models/Characters/Nahida/NPC_Avatar_Loli_Catalyst_Nahida_Edit.fbx#Scene",
    "models/Characters/Nilou/NPC_Avatar_Girl_Sword_Nilou.fbx#Scene",
    "models/Characters/Ningguang/Default/Cs_Avatar_Lady_Catalyst_Ningguang.fbx#Scene",
    "models/Characters/Ningguang/Floral/Avatar_Lady_Catalyst_NingguangCostumeFloral (merge).fbx#Scene",
    "models/Characters/Noelle/NPC_Coop_Avatar_Girl_Claymore_Noel (merge).fbx#Scene",
    "models/Characters/Raiden Shogun/Cs_Avatar_Lady_Pole_Shougun (merge).fbx#Scene",
    "models/Characters/Razor/NPC_Avatar_Boy_Claymore_Razor.fbx#Scene",
    "models/Characters/Rosaria/Default/NPC_Avatar_Lady_Pole_Rosaria (merge).fbx#Scene",
    "models/Characters/Sangonomiya Kokomi/NPC_Homeworld_Avatar_Girl_Catalyst_Kokomi_Edit (merge).fbx#Scene",
    "models/Characters/Sayu/Avatar_Loli_Claymore_Sayu (merge).fbx#Scene",
    "models/Characters/Shenhe/NPC_Avatar_Lady_Pole_Shenhe_Edit (merge).fbx#Scene",
    "models/Characters/Shikanoin Heizou/NPC_Avatar_Boy_Catalyst_Heizo_Edit (merge).fbx#Scene",
    "models/Characters/Sucrose/NPC_Homeworld_Avatar_Girl_Catalyst_Sucrose_Edit (merge).fbx#Scene",
    "models/Characters/Tighnari/Avatar_Boy_Bow_Tighnari.fbx#Scene",
    "models/Characters/Venti/NPC_Avatar_Boy_Bow_Venti.fbx#Scene",
    "models/Characters/Wanderer/Avatar_Boy_Catalyst_Wanderer (merge).fbx#Scene",
    "models/Characters/Xiangling/NPC_Avatar_Girl_Pole_Xiangling_Edit (merge).fbx#Scene",
    "models/Characters/Xiao/Cs_Avatar_Boy_Pole_Xiao.fbx#Scene",
    "models/Characters/Xingqiu/Avatar_Boy_Sword_Xingqiu_Remote (merge).fbx#Scene",
    "models/Characters/Xinyan/NPC_Homeworld_Avatar_Girl_Claymore_Xinyan (merge).fbx#Scene",
    "models/Characters/Yae Miko/Avatar_Lady_Catalyst_Yae_Remote (merge).fbx#Scene",
    "models/Characters/Yanfei/NPC_Homeworld_Avatar_Girl_Catalyst_Feiyan_Edit (merge).fbx#Scene",
    "models/Characters/Yelan/Cs_Avatar_Lady_Bow_Yelan (merge).fbx#Scene",
    "models/Characters/Zhongli/NPC_Avatar_Male_Pole_Zhongli_Edit (merge).fbx#Scene"
];

//#[cfg(feature = "screenshot")]

lazy_static! {
    static ref HEIGHT_OFFSETS: HashMap<String, f32> = {
        HashMap::from([
            ("Lady".to_string(), 0.15),
            ("Girl".to_string(), 0.0),
            ("Loli".to_string(), -0.25),
            ("Male".to_string(), 0.3),
            ("Boy".to_string(), 0.09),
        ])
    };

    static ref SIZE_OVERRIDES: HashMap<String, f32> = {
        HashMap::from([
            ("models/Characters/Kamisato Ayato/Avatar_Male_Sword_Ayato (merge).fbx#Scene".to_string(), 10.0),
            ("models/Characters/Rosaria/Censored/Avatar_Lady_Pole_Rosaria.fbx#Scene".to_string(), 1.0),
            //("models/Characters/Kaveh/NPC_Avatar_Male_Claymore_Kaveh.fbx#Scene".to_string(), 100.0),
            //("models/Characters/Kamisato Ayato/Avatar_Male_Sword_Ayato (merge).fbx#Scene".to_string(), 1.0),
        ])
    };

    static ref LIGHT_OFFSETS: HashMap<String, Vec3> = {
        HashMap::from([
            ("models/Characters/Arataki Itto/Avatar_Male_Claymore_Itto_Remote (merge).fbx#Scene".to_string(), Vec3::new(0.0, 0.0, -10.0)),
            ("models/Characters/Kuki Shinobu/NPC_Homeworld_Avatar_Girl_Sword_Shinobu (merge).fbx#Scene".to_string(), Vec3::new(0.0, 0.0, -10.0)),
            ("models/Characters/Kaveh/NPC_Avatar_Male_Claymore_Kaveh.fbx#Scene".to_string(), Vec3::new(0.0, 0.0, -10.0)),
            ("models/Characters/Diluc/Default/Cs_Avatar_Male_Claymore_Diluc.fbx#Scene".to_string(), Vec3::new(0.0, 0.0, -10.0)),
            ("models/Characters/Diluc/Flamme/Avatar_Male_Claymore_DilucCostumeFlamme.fbx#Scene".to_string(), Vec3::new(0.0, 0.0, -10.0)),
        ])
    };
}

struct GeneratePlugin;

#[cfg(feature = "screenshot")]
impl Plugin for GeneratePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(screenshot_on_f12)
            .add_system_set(SystemSet::on_update(AppState::Ready).with_system(generate));
    }
}

#[cfg(not(feature = "screenshot"))]
impl Plugin for GeneratePlugin {
    fn build(&self, app: &mut App) {}
}

struct UiPlugin;

#[cfg(feature = "ui")]
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin).add_system(ui_system);
            //.add_plugin(EditorPlugin);
    }
}

#[cfg(not(feature = "ui"))]
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {}
}

fn main() {
    App::new()
        //.insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_state(AppState::Loading)
        //.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        // .add_plugin(DebugLinesPlugin::default())
        .insert_resource(FbxMaterialLoaders::<CelMaterial>(vec![
            &load_cel_material,
            &load_cel_material_fallback,
        ]))
        .add_plugin(RonAssetPlugin::<PresetsGroups>::new(&["ron"]))
        //.add_plugin(FbxPlugin)
        .init_asset_loader::<FbxLoader<CelMaterial>>()
        .add_asset::<FbxMesh<CelMaterial>>()
        .add_asset::<FbxScene<CelMaterial>>()
        .add_plugin(MaterialPlugin::<CelMaterial>::default())
        .add_plugin(CelShaderPlugin)
        .add_system(pan_orbit_camera)
        .add_plugin(UiPlugin)
        .add_startup_system(setup)
        .add_system(initialize)
        .add_plugin(GeneratePlugin)
        .add_system(hotkey_system)
        .add_system(update_face_direction)
        //.add_system(axis_lines)
        .add_system(rotate_character_system)
        .run();
}

fn rotate_character_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        With<CurrentCharacter>
    )>,
) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(time.elapsed_seconds() as f32);
    }
}

fn update_face_direction(
    mut query: Query<(
        &mut Transform,
        With<CurrentCharacter>
    )>,
    mut materials_query: Query<&mut Handle<CelMaterial>>,
    mut materials: ResMut<Assets<CelMaterial>>,
) {
    if let Ok(trans) = query.get_single() {
        let face_direction = trans.0.rotation;

        materials_query.for_each_mut(|mat| {
            if let Some(ref mut mat) = materials.get_mut(mat.as_ref()) {
                mat.update_head_direction(face_direction.into())
            }
        });
    }
}

#[derive(PartialEq)]
enum ScreenshotState {
    SetupCamera,
    ScreenhotA,
    ScreenhotB,
    Done
}

impl Default for ScreenshotState {
    fn default() -> Self {
        Self::SetupCamera
    }
}

#[cfg(feature = "screenshot")]
fn generate(
    mut counter: Local<usize>,
    mut skip_counter: Local<usize>,
    mut character_counter: Local<usize>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut presets_handle: Option<ResMut<PresetsResource>>,
    mut presets: ResMut<Assets<PresetsGroups>>,
    mut camera_query: Query<(&mut Transform, With<Camera>)>,
    mut light_query: Query<(&mut Transform, With<PointLight>, Without<Camera>)>,
    mut materials_query: Query<&mut Handle<CelMaterial>>,
    mut materials: ResMut<Assets<CelMaterial>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut current_character_query: Query<(Entity, &mut Visibility), With<CurrentCharacter>>,
    mut app_state: ResMut<State<AppState>>,
    mut state: Local<ScreenshotState>
) {
    if *state == ScreenshotState::Done {
        return;
    }

    *skip_counter += 1;
    if *skip_counter < 2 && *skip_counter % 2 == 0 {
        return;
    }

    let sample_size = 119;

    assert!(sample_size & 1 == 1);

    if *counter > sample_size {
        *character_counter += 1;
        *counter = 0;

        if *character_counter == CHARACTERS.len() {
            println!("done!");
            *character_counter += 1;
            return;
        } else if *character_counter > CHARACTERS.len() {
            return;
        }

        //commands.entity(current_character_query.single()).despawn();
        current_character_query
            .for_each_mut(|mut c| c.1.set(Box::new(Visibility::Hidden)).unwrap());

        app_state.as_mut().set(AppState::Loading).unwrap();
        // commands.spawn((
        //     SceneBundle {
        //         scene: asset_server.load(CHARACTERS[*character_counter]),
        //         //.load("models/Characters/Faruzan/Avatar_Girl_Bow_Faruzan (merge).fbx#Scene"),
        //         transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
        //         //scene: asset_server.load("models/cube.fbx#Scene"),//.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
        //         ..default()
        //     },
        //     CurrentCharacter,
        // ));
        spawn_character(&mut commands, &asset_server, CHARACTERS[*character_counter]);

        return;
    }

    guard! { let Some(presets_handle) = presets_handle else { return; } }
    if let Some(ref mut presets) = presets.get_mut(&presets_handle.0) {
        guard! { let Some(presets) = presets.groups.get_mut("face2.5") else { return } }

        //let change_state = *counter & 1 == 0;
        match *state {
            ScreenshotState::SetupCamera => {
                if *character_counter >= CHARACTERS.len() {
                    *state = ScreenshotState::Done;
                    return;
                }

                let mut rng = rand::thread_rng();
                let camera = rng.gen_range(0..presets.camera.len());
                let camera = presets.camera[camera].clone();

                let light = rng.gen_range(0..presets.light.len());
                let mut light = presets.light[light].clone();
                light.translation += LIGHT_OFFSETS.get(CHARACTERS[*character_counter]).map(|o| *o).unwrap_or(Vec3::ZERO);

                let mut scene_camera = camera_query.single_mut().0;
                *scene_camera.as_mut() = camera;
                scene_camera.translation.y += get_height_offset(CHARACTERS[*character_counter]);

                *light_query.single_mut().0.as_mut() = light;

                *state = ScreenshotState::ScreenhotA;
            }
            ScreenshotState::ScreenhotA | ScreenshotState::ScreenhotB => {
                materials_query.for_each_mut(|mat| {
                    if let Some(ref mut mat) = materials.get_mut(mat.as_ref()) {
                        mat.diffuse_only = *state == ScreenshotState::ScreenhotA;
                    }
                });

                let path = format!("./output/sample-{}-{}.png", *character_counter, *counter);
                *counter += 1;
                screenshot_manager
                    .save_screenshot_to_disk(WindowId::primary(), path)
                    .unwrap();

                if *state == ScreenshotState::ScreenhotA {
                    *state = ScreenshotState::ScreenhotB;
                } else {
                    *state = ScreenshotState::SetupCamera;
                }
            }
            ScreenshotState::Done => {}
        }
    }
}

#[cfg(feature = "screenshot")]
fn screenshot_on_f12(
    input: Res<Input<KeyCode>>,
    //main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if input.just_pressed(KeyCode::F12) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        screenshot_manager
            .save_screenshot_to_disk(WindowId::primary(), path)
            .unwrap();
    }
}

#[cfg(feature = "ui")]
fn hotkey_system(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut current_character_query: Query<Entity, With<CurrentCharacter>>,
    mut materials_query: Query<&mut Handle<CelMaterial>>,
    mut materials: ResMut<Assets<CelMaterial>>,
    mut is_diffuse_only: Local<bool>
) {
    if input.just_pressed(KeyCode::D) {
        if let Ok(entity) = current_character_query.get_single() {
            commands.entity(entity).despawn();
        }
    }

    if input.just_pressed(KeyCode::F) {
        *is_diffuse_only = !*is_diffuse_only;
        materials_query.for_each_mut(|mat| {
            if let Some(ref mut mat) = materials.get_mut(mat.as_ref()) {
                mat.diffuse_only = *is_diffuse_only;
            }
        });
    }
}

fn get_height_offset(name: &str) -> f32 {
    HEIGHT_OFFSETS
        .iter()
        .find(|o| name.contains(o.0))
        .map(|o| *o.1)
        .unwrap_or(0.0)
}

#[derive(Default)]
struct UIState {
    hidden: bool,
    filter: String,
    preset_name: String,
    offset: f32,
}

#[cfg(feature = "ui")]
fn ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut egui_context: ResMut<EguiContext>,
    mut presets_handle: Option<ResMut<PresetsResource>>,
    mut presets_groups: ResMut<Assets<PresetsGroups>>,
    mut camera_query: Query<(&mut Transform, With<Camera>)>,
    mut light_query: Query<(&mut Transform, With<PointLight>, Without<Camera>)>,
    mut current_character_query: Query<(Entity, &mut Visibility, &Name), With<CurrentCharacter>>,
    mut state: Local<UIState>,
    mut current_presets: Local<Presets>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Insert) { 
        state.hidden = !state.hidden;
    }

    if state.hidden {
        return;
    }

    use std::ops::Deref;

    use bevy_egui::egui::{Label, Sense};

    guard! { let Some(presets_handle) = presets_handle else { return; } }
    //guard! {  else { return; } }

    //let presets = presets_groups..get(state.preset_name);

    //if let Some(ref mut presets) = presets.get_mut(&presets_handle.0) {
    guard! {let Some(presets_groups) = presets_groups.get_mut(&presets_handle.0) else { return; }}

    // let presets = {
    //     if let Some(current) = presets_groups.groups.get_mut(&state.preset_name) {
    //         current
    //     } else {
    //         &mut current_presets
    //     }
    // };

    let mut presets = current_presets;

    let selected = current_character_query.iter_mut().find(|c| c.1.is_visible);
    guard! { let Some(mut selected) = selected else {
        return;
    }}

    egui::Window::new("Cameras")
        .vscroll(true)
        .hscroll(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("y offset: ");
                ui.add(egui::DragValue::new(&mut state.offset).speed(0.1));
            });
            for camera in &presets.camera {
                if ui
                    .add(
                        Label::new(format!("{:?} {:?}", camera.translation, camera.rotation))
                            .sense(Sense::click()),
                    )
                    .clicked()
                {
                    *camera_query.single_mut().0.as_mut() = (*camera).clone();
                    camera_query.single_mut().0.translation.y +=
                        get_height_offset(selected.2.as_str()); //state.offset;
                                                                //println!("offset {}", get_height_offset(selected.2.as_str()));
                }
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

    egui::Window::new("Lights")
        .vscroll(true)
        .show(egui_context.ctx_mut(), |ui| {
            for light in &presets.light {
                ui.label(format!("{:?}", light.translation));
            }
            if ui.button("Add").clicked() {
                presets.light.push(light.clone());
            }
        });

    egui::Window::new("Persistance").show(egui_context.ctx_mut(), |ui| {
        egui::ScrollArea::new([true, false]).show(ui, |ui| {
            for group in presets_groups.groups.keys() {
                if ui
                    .add(Label::new(format!("{:?}", group)).sense(Sense::click()))
                    .clicked()
                {
                    state.preset_name = group.clone();
                    *presets = presets_groups.groups.get(group).unwrap().clone();
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label("Preset name: ");
            ui.text_edit_singleline(&mut state.preset_name);
        });
        if ui.button("Add presets").clicked() {
            presets_groups
                .groups
                .insert(state.preset_name.clone(), (*presets).clone());
        }
        if ui.button("Save file").clicked() {
            //let registry = type_registry.read();
            //let serializer = ReflectSerializer::new(presets.as_ref(), &registry);
            let serialized =
                ron::ser::to_string_pretty(&presets_groups, ron::ser::PrettyConfig::default())
                    .unwrap();

            IoTaskPool::get()
                .spawn(async move {
                    // Write the scene RON data to file
                    File::create(format!("assets/{}", PRESETS_FILENAME))
                        .and_then(|mut file| file.write(serialized.as_bytes()))
                        .expect("Error while writing scene to file");
                })
                .detach();
        }
    });

    egui::Window::new("Characters").show(egui_context.ctx_mut(), |ui| {
        ui.text_edit_singleline(&mut state.filter);
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(format!("selected: {:?}", selected.2.as_str()));
            //ui.label(format!("loaded characters: {:?}", current_character_query.iter().count()));
            for name in CHARACTERS {
                if !name
                    .to_ascii_lowercase()
                    .contains(state.filter.to_ascii_lowercase().as_str())
                {
                    continue;
                }
                if ui
                    .add(Label::new(format!("{:?}", name)).sense(Sense::click()))
                    .clicked()
                {
                    selected.1.is_visible = false;
                    //commands.entity(selected.0).despawn();
                    spawn_character(&mut commands, &asset_server, name)
                }
            }
        });
    });
}

// fn axis_lines(mut lines: ResMut<DebugLines>) {
//     lines.line_colored(
//         Vec3::new(0.0, 1.0, 0.0),
//         Vec3::new(4.0, 1.0, 0.0),
//         0.0,
//         Color::RED,
//     );
//     lines.line_colored(
//         Vec3::new(0.0, 1.0, 0.0),
//         Vec3::new(0.0, 1.0, 4.0),
//         0.0,
//         Color::BLUE,
//     );
// }

fn initialize(
    mut ev_asset: EventReader<AssetEvent<FbxScene<CelMaterial>>>,
    mut query_visibility: Query<(&Handle<CelMaterial>, &mut Visibility)>,
    mut app_state: ResMut<State<AppState>>,
    materials: Res<Assets<CelMaterial>>
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle: _ } => {
                println!("scene loaded, meshes count: {}", query_visibility.iter().count());

                query_visibility
                    .iter_mut()
                    .filter(|(mat, _)| materials.get(mat).map(|mat| mat.diffuse_only).unwrap_or(true))
                    .for_each(|mut vis| {
                        #[cfg(feature = "screenshot")]
                        vis.1.set(Box::new(Visibility::Hidden)).unwrap();
                        println!("hiding effect mesh");
                        #[cfg(feature = "ui")]
                        {
                            vis.1.is_visible = false;
                        }
                    });

                if *app_state.as_ref().current() != AppState::Ready {
                    app_state.as_mut().set(AppState::Ready).unwrap();
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

fn spawn_character(commands: &mut Commands, asset_server: &Res<AssetServer>, name: &str) {
    let size = SIZE_OVERRIDES.get(name).map(|v| *v).unwrap_or(100.0);

    commands.spawn((
        SceneBundle {
            scene: asset_server.load(name), //"models/Characters/Faruzan/Avatar_Girl_Bow_Faruzan (merge).fbx#Scene"),
            //.load("models/Characters/Faruzan/Avatar_Girl_Bow_Faruzan (merge).fbx#Scene"),
            transform: Transform::from_scale(Vec3::new(size, size, size)),
            //scene: asset_server.load("models/cube.fbx#Scene"),//.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
            ..default()
        },
        CurrentCharacter,
        Name::new(name.to_string()),
    ));
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _custom_materials: ResMut<Assets<CelMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());
    window.set_resolution(512.0, 512.0);
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

    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(shape::UVSphere{ radius: 20.0, ..Default::default()}.into()),
    //     material: test_materials.add(TestMaterial {
    //         matcap: Some(asset_server.load("models/Characters/Diona/Avatar_Tex_MetalMap.png"))
    //     }),
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
    //     transform: Transform::from_xyz(-0.5, 1.5, 1.0)
    //         .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    //     ..default()
    // });
    spawn_camera(&mut commands);

    #[cfg(feature = "screenshot")]
    let character = CHARACTERS[0];
    #[cfg(feature = "ui")]
    let character = "models/Characters/Amber/Default/Avatar_Girl_Bow_Ambor.fbx#Scene";

    spawn_character(&mut commands, &asset_server, character);

    commands.insert_resource(PresetsResource(asset_server.load(PRESETS_FILENAME)));
}
