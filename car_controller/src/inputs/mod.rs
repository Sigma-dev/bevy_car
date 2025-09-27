use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct CarControllerInputPlugin;

impl Plugin for CarControllerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_car_controller_input);
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CarControllerInputs {
    pub(crate) forward: bool,
    pub(crate) backward: bool,
    pub(crate) left: bool,
    pub(crate) right: bool,
}

impl CarControllerInputs {
    pub fn from_keyboard(keyboard: &ButtonInput<KeyCode>) -> Self {
        Self {
            forward: keyboard.pressed(KeyCode::KeyW),
            backward: keyboard.pressed(KeyCode::KeyS),
            left: keyboard.pressed(KeyCode::KeyA),
            right: keyboard.pressed(KeyCode::KeyD),
        }
    }
}

impl CarControllerInputs {
    pub fn new() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Component, Debug)]
pub struct CarControllerInput {
    inputs: CarControllerInputs,
    outside_controlled: bool,
}

impl CarControllerInput {
    pub fn new() -> Self {
        Self {
            inputs: CarControllerInputs::new(),
            outside_controlled: false,
        }
    }

    pub fn new_controlled() -> Self {
        Self {
            inputs: CarControllerInputs::new(),
            outside_controlled: true,
        }
    }

    pub fn update(&mut self, inputs: CarControllerInputs) {
        self.inputs = inputs;
    }

    pub fn get_inputs(&self) -> &CarControllerInputs {
        &self.inputs
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
            continue;
        }
        car_controller_input.update(CarControllerInputs::from_keyboard(&keyboard));
    }
}
