use bevy::reflect::TypeUuid;
use bevy::tasks::IoTaskPool;
use bevy::utils::HashMap;
use bevy::window::WindowId;
use bevy::{asset::load_internal_asset, prelude::*};
//use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use camera_control::{pan_orbit_camera, spawn_camera};
use experiment_shaders::TestMaterial;

mod camera_control;
mod experiment_shaders;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }).set(WindowPlugin {
            window: WindowDescriptor {
                // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                transparent: true,
                // Disabling window decorations to make it feel more like a widget than a window
                decorations: false,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugin(MaterialPlugin::<TestMaterial>::default())
        .add_system(pan_orbit_camera)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut test_materials: ResMut<Assets<TestMaterial>>,
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

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(
            shape::UVSphere {
                radius: 0.25,
                ..Default::default()
            }
            .into(),
        ),
        material: test_materials.add(TestMaterial {
            matcap: Some(asset_server.load("metal_map.png")),
        }),
        ..default()
    });

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

    spawn_camera(&mut commands);
}
