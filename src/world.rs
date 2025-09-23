use avian3d::prelude::*;
use bevy::prelude::*;
use numpad_cameras::NumpadCamera;

#[derive(Component)]
struct Ground;

pub fn spawn_world(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        NumpadCamera::new(KeyCode::Numpad0),
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
}
