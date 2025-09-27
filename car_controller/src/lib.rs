use bevy::prelude::*;
use force_accumulator::prelude::*;

use crate::{
    inputs::{CarControllerInput, CarControllerInputPlugin},
    vertical_suspension::VerticalSuspensionPlugin,
    wheels::CarWheelPlugin,
};

pub mod debug;
pub mod inputs;
pub mod prelude;
mod vertical_suspension;
mod wheels;
pub struct CarControllerPlugin;

impl Plugin for CarControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CarWheelPlugin,
            CarControllerInputPlugin,
            VerticalSuspensionPlugin,
            ForceAccumulatorPlugin,
        ))
        .add_systems(Update, (on_car_controller_added, handle_turning));
    }
}

#[derive(Component, Debug)]
#[require(CarControllerInput)]
pub struct CarController {
    pub steering_speed: f32,
    pub steer_angle: f32,
    pub max_steer_angle: f32,
}

impl CarController {
    pub fn new() -> Self {
        Self {
            steering_speed: 1.,
            steer_angle: 0.0,
            max_steer_angle: 30_f32.to_radians(),
        }
    }
}

#[derive(Component)]
pub struct CarControllerEngine {
    power: f32,
}

impl CarControllerEngine {
    pub fn new() -> Self {
        Self { power: 2.0 }
    }

    pub fn get_power(&self) -> f32 {
        self.power
    }
}

fn on_car_controller_added(
    mut commands: Commands,
    car_controller: Query<Entity, Added<CarController>>,
) {
    for car_controller in car_controller.iter() {
        commands
            .entity(car_controller)
            .insert(ForceAccumulator::new());
    }
}

fn handle_turning(
    time: Res<Time>,
    mut car_controller: Query<(&mut CarController, &CarControllerInput)>,
) {
    for (mut car_controller, car_controller_input) in car_controller.iter_mut() {
        let input = car_controller_input.get_inputs();
        let input = if input.left {
            1.0
        } else if input.right {
            -1.0
        } else {
            continue;
        };

        car_controller.steer_angle += input * time.delta_secs() * car_controller.steering_speed;
        car_controller.steer_angle = car_controller.steer_angle.clamp(
            -car_controller.max_steer_angle,
            car_controller.max_steer_angle,
        );
    }
}
