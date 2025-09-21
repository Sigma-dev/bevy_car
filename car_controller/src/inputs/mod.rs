use bevy::prelude::*;

pub struct CarControllerInputPlugin;

impl Plugin for CarControllerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_car_controller_input);
    }
}

#[derive(Component)]
pub struct CarControllerInput {
    pub(crate) forward: bool,
    pub(crate) backward: bool,
    pub(crate) left: bool,
    pub(crate) right: bool,
    outside_controlled: bool,
}

impl CarControllerInput {
    pub fn new() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,
            outside_controlled: false,
        }
    }

    pub fn new_controlled() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,
            outside_controlled: true,
        }
    }
}

impl Default for CarControllerInput {
    fn default() -> Self {
        Self::new()
    }
}

fn update_car_controller_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut car_controller_input: Query<&mut CarControllerInput>,
) {
    for mut car_controller_input in car_controller_input.iter_mut() {
        if car_controller_input.outside_controlled {
            return;
        }
        car_controller_input.forward = keyboard.pressed(KeyCode::KeyW);
        car_controller_input.backward = keyboard.pressed(KeyCode::KeyS);
        car_controller_input.left = keyboard.pressed(KeyCode::KeyA);
        car_controller_input.right = keyboard.pressed(KeyCode::KeyD);
    }
}
