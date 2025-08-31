use bevy::prelude::*;

pub mod prelude {
    pub use crate::{NumpadCamera, NumpadCamerasPlugin};
}

pub struct NumpadCamerasPlugin;

const NUMPAD_KEYS: [KeyCode; 10] = [
    KeyCode::Numpad0,
    KeyCode::Numpad1,
    KeyCode::Numpad2,
    KeyCode::Numpad3,
    KeyCode::Numpad4,
    KeyCode::Numpad5,
    KeyCode::Numpad6,
    KeyCode::Numpad7,
    KeyCode::Numpad8,
    KeyCode::Numpad9,
];

impl Plugin for NumpadCamerasPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentNumpadCamera(KeyCode::Numpad0))
            .add_systems(PreUpdate, handle_numpad_cameras)
            .add_systems(PreUpdate, handle_current_numpad_camera);
    }
}

#[derive(Resource)]
pub struct CurrentNumpadCamera(KeyCode);

#[derive(Component)]
pub struct NumpadCamera {
    numpad_key: KeyCode,
}

impl NumpadCamera {
    pub fn new(numpad_key: KeyCode) -> Self {
        if !NUMPAD_KEYS.contains(&numpad_key) {
            panic!("Invalid numpad key: {:?}", numpad_key);
        }
        Self { numpad_key }
    }
}

fn handle_current_numpad_camera(
    mut current_numpad_camera: ResMut<CurrentNumpadCamera>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for key in NUMPAD_KEYS {
        if keyboard.just_pressed(key) {
            current_numpad_camera.0 = key;
        }
    }
}

fn handle_numpad_cameras(
    current_numpad_camera: Res<CurrentNumpadCamera>,
    mut numpad_cameras: Query<(&mut Camera, &NumpadCamera)>,
) {
    for (mut camera, numpad_camera) in numpad_cameras.iter_mut() {
        camera.is_active = numpad_camera.numpad_key == current_numpad_camera.0;
    }
}
