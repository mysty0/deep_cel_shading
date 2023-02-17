use bevy::prelude::*;
use bevy_mod_fbx::FbxPlugin;
use bevy_editor_pls::prelude::*;

#[derive(Component)]
pub struct Spin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FbxPlugin)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_system(spin_cube)
        .run();
}

fn spin_cube(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_y(0.3 * time.delta_seconds());
        transform.rotate_local_x(0.3 * time.delta_seconds());
        transform.rotate_local_z(0.3 * time.delta_seconds());
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
            transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
            //scene: asset_server.load("models/cube.fbx#Scene"),//.load("models/fischl/Avatar_Girl_Bow_FischlCostumeHighness.fbx#Scene"),
            ..default()
        },
        Spin,
    ));
}