use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_steam_p2p::prelude::*;
use car_controller::prelude::*;
use numpad_cameras::NumpadCamera;

pub fn spawn_car(
    commands: &mut Commands,
    asset_server: &AssetServer,
    transform: Transform,
    network_identity: NetworkIdentity,
) {
    let car = commands
        .spawn((
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cars/truck.glb")),
            ),
            CarController,
            RigidBody::Dynamic,
            CenterOfMass::ZERO,
            transform,
            Visibility::Inherited,
            network_identity,
            children![
                (
                    Collider::cuboid(2., 1., 5.),
                    Transform::from_xyz(0.0, 1., 0.0)
                ),
                (
                    Camera3d::default(),
                    NumpadCamera::new(KeyCode::Numpad0),
                    Transform::from_xyz(-0.3, 1.4, 0.0),
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
                ),
                (
                    Camera3d::default(),
                    NumpadCamera::new(KeyCode::Numpad3),
                    Transform::from_xyz(0.0, 5., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
                ),
            ],
        ))
        .id();

    for i in 0..4 {
        let right = i % 2 == 0;
        let front = i / 2 == 0;
        commands.spawn((
            ChildOf(car),
            Mass(1.),
            CarWheel::new(if front { 3. } else { 0.0 }, 0.2, 0.05, front),
            VerticalSuspension::new(10., 0.5, 0.6),
            Transform::from_xyz(
                if right { 1. } else { -1. } * 0.75,
                0.1,
                if front { -1. } else { 1. } * 1.7,
            ),
            Visibility::Inherited,
            children![(
                CarWheelVisuals::new(0.4),
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("models/wheels/wheel.glb"))
                ),
            )],
        ));
    }
}
