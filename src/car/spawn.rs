use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_steam_p2p::prelude::*;
use car_controller::prelude::*;
use fps_camera::FpsCamera;
use numpad_cameras::NumpadCamera;

use crate::car::{inputs::RemotelyControlled, steering_wheel::SteeringWheel};

pub fn spawn_car(
    commands: &mut Commands,
    asset_server: &AssetServer,
    transform: Transform,
    network_identity: NetworkIdentity,
    remotely_controlled_by: Option<SteamId>,
    is_lobby_owner: bool,
    is_local: bool,
) {
    let car = commands
        .spawn((
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cars/truck.glb")),
            ),
            CarController::new(),
            RigidBody::Dynamic,
            CenterOfMass::ZERO,
            transform,
            Visibility::Inherited,
            network_identity,
            NetworkedTransform::new(true, true, false),
            children![(
                Visibility::Inherited,
                Transform::from_xyz(-0.3, 1.25, -0.5).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    22_5_f32.to_radians(),
                    0_f32.to_radians(),
                    180_f32.to_radians(),
                )),
                children![(
                    SceneRoot(
                        asset_server.load(
                            GltfAssetLabel::Scene(0)
                                .from_asset("models/steering_wheels/steering_wheel.glb")
                        ),
                    ),
                    SteeringWheel {
                        rotation_multiplier: 12.0,
                    },
                ),],
            )],
        ))
        .id();

    if let Some(steam_id) = remotely_controlled_by {
        commands.entity(car).insert((
            CarControllerInput::new_controlled(),
            RemotelyControlled(steam_id),
        ));
    } else if is_lobby_owner {
        commands.entity(car).insert(CarControllerInput::new());
    } else {
        commands
            .entity(car)
            .insert(CarControllerInput::new_controlled());
    }

    if is_local {
        commands.entity(car).with_children(|parent| {
            parent.spawn((
                Collider::cuboid(2., 1., 5.),
                Transform::from_xyz(0.0, 1., 0.0),
            ));
            parent.spawn((
                Camera3d::default(),
                NumpadCamera::new(KeyCode::Numpad1),
                Transform::from_xyz(0.0, 0., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
            parent.spawn((
                Camera3d::default(),
                NumpadCamera::new(KeyCode::Numpad2),
                Transform::from_xyz(5., 0., 1.5).looking_at(Vec3::new(0., 0., 1.5), Vec3::Y),
            ));
            parent.spawn((
                Camera3d::default(),
                NumpadCamera::new(KeyCode::Numpad3),
                Transform::from_xyz(0.0, 5., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
            parent.spawn((
                Camera3d::default(),
                NumpadCamera::new(KeyCode::Numpad4),
                Transform::from_xyz(-0.3, 1.4, 0.0),
                FpsCamera::new(0.1),
            ));
        });
    }

    for i in 0..4 {
        let right = i % 2 == 0;
        let front = i / 2 == 0;
        commands.spawn((
            ChildOf(car),
            Mass(1.),
            CarWheel::new(if front { 2. } else { 0.0 }, 0.15, 0.05, front),
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
