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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        NumpadCamera::new(KeyCode::Numpad4),
        Transform::from_xyz(0.0, 1.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    /*  let test = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial::default())),
            Transform::from_xyz(0.0, 3.0, 0.0),
            Collider::cuboid(1.0, 1.0, 1.0),
            RigidBody::Dynamic,
        ))
        .id();

    commands.spawn((
        VerticalSuspension::new(30.0, 5.),
        Suspension(test),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
    )); */

    let car = commands
        .spawn((
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cars/test_car.glb")),
            ),
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
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
            VerticalSuspension::new(30.0, 5.),
            Suspension(car),
            Transform::from_xyz(
                if right { 0.75 } else { -0.75 },
                1.0,
                if front { 1.5 } else { -1.5 },
            ),
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("models/wheels/test_wheel.glb")),
            ),
        ));
    }

    /*
    let spring_stiffness = 10.;
    let damping_ratio = 0.5;
    let origin_offset = 0.3;
    let height = -0.2;
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cars/test_car.glb")),
        ),
        RigidBody::Dynamic,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::Inherited,
        children![
            (
                Camera3d::default(),
                NumpadCamera::new(KeyCode::Numpad1),
                Transform::from_xyz(0.0, 0., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ),
            (
                Camera3d::default(),
                NumpadCamera::new(KeyCode::Numpad2),
                Transform::from_xyz(2., 0., 1.5).looking_at(Vec3::new(0., 0., 1.5), Vec3::Y),
            ),
            (
                CarWheel::new(origin_offset, 0.15, spring_stiffness, damping_ratio),
                Transform::from_xyz(0.75, height, 1.5),
                Name::new("BR") /*    children![SceneRoot(asset_server.load(
                                    GltfAssetLabel::Scene(0).from_asset("models/wheels/test_wheel.glb")
                                ),),], */
            ),
            (
                CarWheel::new(origin_offset, 0.15, spring_stiffness, damping_ratio),
                Transform::from_xyz(-0.75, height, 1.5),
                Name::new("BL") /*    children![SceneRoot(asset_server.load(
                                    GltfAssetLabel::Scene(0).from_asset("models/wheels/test_wheel.glb")
                                ),),], */
            ),
            (
                CarWheel::new(origin_offset, 0.15, spring_stiffness, damping_ratio),
                Transform::from_xyz(-0.75, height, -1.5),
                Name::new("FL") /*       children![SceneRoot(asset_server.load(
                                    GltfAssetLabel::Scene(0).from_asset("models/wheels/test_wheel.glb")
                                ),),], */
            ),
            (
                CarWheel::new(origin_offset, 0.15, spring_stiffness, damping_ratio),
                Transform::from_xyz(0.75, height, -1.5),
                Name::new("FR") /*       children![SceneRoot(asset_server.load(
                                    GltfAssetLabel::Scene(0).from_asset("models/wheels/test_wheel.glb")
                                ),),], */
            )
        ],
    )); */
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
