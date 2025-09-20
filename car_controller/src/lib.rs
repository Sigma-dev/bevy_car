use bevy::prelude::*;
use force_accumulator::prelude::*;

use crate::{vertical_suspension::VerticalSuspensionPlugin, wheels::CarWheelPlugin};

pub mod debug;
pub mod prelude;
mod vertical_suspension;
mod wheels;
pub struct CarControllerPlugin;

impl Plugin for CarControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CarWheelPlugin,
            VerticalSuspensionPlugin,
            ForceAccumulatorPlugin,
        ))
        .add_systems(Update, on_car_controller_added);
    }
}

#[derive(Component)]
pub struct CarController;

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
