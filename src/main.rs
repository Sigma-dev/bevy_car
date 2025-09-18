use avian3d::prelude::*;
use bevy::prelude::*;
use car_controller::prelude::*;
use numpad_cameras::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_plugins((
            NumpadCamerasPlugin,
            CarControllerPlugin,
            CarControllerDebugPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_ground)
        .run()
}

#[derive(Component)]
struct Ground;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        NumpadCamera::new(KeyCode::Numpad4),
        Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/maps/test_map.glb")),
        ),
        Ground,
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Visibility::Hidden,
    ));

    let car = commands
        .spawn((
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cars/truck.glb")),
            ),
            CarController,
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
            CenterOfMass::ZERO,
            Transform::from_xyz(0.0, 1.5, 0.0),
            Visibility::Inherited,
            children![
                (
                    Camera3d::default(),
                    NumpadCamera::new(KeyCode::Numpad0),
                    Transform::from_xyz(0.0, 5., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
                ),
                (
                    Camera3d::default(),
                    NumpadCamera::new(KeyCode::Numpad1),
                    Transform::from_xyz(0.0, 0., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
                ),
                (
                    Camera3d::default(),
                    NumpadCamera::new(KeyCode::Numpad2),
                    Transform::from_xyz(5., 0., 1.5).looking_at(Vec3::new(0., 0., 1.5), Vec3::Y),
                )
            ],
        ))
        .id();

    for i in 0..4 {
        let right = i % 2 == 0;
        let front = i / 2 == 0;
        commands.spawn((
            ChildOf(car),
            Mass(1.),
            CarWheel::new(if front { 0.6 } else { 0.0 }, 0.2, 0., front),
            VerticalSuspension::new(3., 0.5, 1.),
            Transform::from_xyz(
                if right { 1. } else { -1. } * 0.75,
                0.1,
                if front { -1. } else { 1. } * 1.7,
            ),
        ));
    }
}

fn move_ground(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ground: Query<&mut Transform, With<Ground>>,
) {
    if keyboard.pressed(KeyCode::PageUp) {
        for mut transform in ground.iter_mut() {
            transform.translation.y += 0.01;
        }
    }
    if keyboard.pressed(KeyCode::PageDown) {
        for mut transform in ground.iter_mut() {
            transform.translation.y -= 0.01;
        }
    }
}
