use avian3d::prelude::*;
use bevy::{light::light_consts::lux, prelude::*};
use numpad_cameras::NumpadCamera;

use crate::camera::CameraBundle;

#[derive(Component)]
struct Ground;

pub fn spawn_world(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.,
        affects_lightmapped_meshes: true,
    });
    let top_down_camera_position = Vec3::new(75.0, 200.0, 25.0);
    commands.spawn((
        CameraBundle::new(),
        NumpadCamera::new(KeyCode::Numpad0),
        Transform::from_translation(top_down_camera_position)
            .looking_at(top_down_camera_position.with_y(0.), -Vec3::Z),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/maps/test_track.glb")),
        ),
        Ground,
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));
}
