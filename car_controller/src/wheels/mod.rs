use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    CarController, CarControllerEngine, inputs::CarControllerInput,
    wheels::visuals::CarWheelVisualsPlugin,
};

pub mod visuals;
pub struct CarWheelPlugin;

impl Plugin for CarWheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                handle_turning,
                handle_power,
                handle_traction,
                handle_rolling_resistance,
            ),
        )
        .add_plugins(CarWheelVisualsPlugin);
    }
}

#[derive(Component)]
pub struct CarWheel {
    grip: f32,
    rolling_resistance: f32,
    is_powered: bool,
    can_turn: bool,
}

impl CarWheel {
    pub fn new(grip: f32, rolling_resistance: f32, can_turn: bool, is_powered: bool) -> Self {
        Self {
            is_powered,
            grip,
            rolling_resistance,
            can_turn,
        }
    }
}

fn handle_rolling_resistance(
    wheels: Query<(&GlobalTransform, &CarWheel, &ChildOf)>,
    mut parents: Query<Forces>,
) {
    {
        for (global_transform, wheel, suspension) in wheels.iter() {
            let Ok(mut forces) = parents.get_mut(suspension.0) else {
                continue;
            };
            let velocity = forces.velocity_at_point(global_transform.translation());
            let rolling_resistance_force = -velocity * wheel.rolling_resistance;

            forces.apply_linear_impulse_at_point(
                rolling_resistance_force,
                global_transform.translation(),
            );
        }
    }
}

fn handle_turning(
    mut wheels: Query<(&mut Transform, &CarWheel, &ChildOf)>,
    parents: Query<&CarController>,
) {
    for (mut transform, wheel, child_of) in wheels.iter_mut() {
        if !wheel.can_turn {
            continue;
        }
        let car_controller = parents.get(child_of.0).unwrap();
        transform.rotation = Quat::from_rotation_y(car_controller.steer_angle);
    }
}

fn handle_power(
    wheels: Query<(&GlobalTransform, &CarWheel, &ChildOf)>,
    mut cars: Query<(Forces, &CarControllerInput, &CarControllerEngine)>,
) {
    for (global_transform, wheel, suspension) in wheels.iter() {
        if wheel.is_powered == false {
            continue;
        }
        let Ok((mut forces, car_controller_input, car_controller_engine)) =
            cars.get_mut(suspension.0)
        else {
            continue;
        };

        let inputs = car_controller_input.get_inputs();
        let input = if inputs.forward {
            1.0
        } else if inputs.backward {
            -1.0
        } else {
            continue;
        };

        let force = global_transform.forward() * car_controller_engine.get_power() * input;
        forces.apply_linear_impulse_at_point(force, global_transform.translation());
    }
}

fn handle_traction(
    time: Res<Time>,
    wheels: Query<(&GlobalTransform, &CarWheel, &ChildOf)>,
    mut cars: Query<Forces>,
) {
    for (global_transform, wheel, suspension) in wheels.iter() {
        let Ok(mut forces) = cars.get_mut(suspension.0) else {
            continue;
        };
        let steering_dir = global_transform.right().as_vec3();
        let velocity = forces.velocity_at_point(global_transform.translation());
        let steering_vel = steering_dir.dot(velocity);
        let desired_vel_change = -steering_vel * wheel.grip;
        let desired_accel = desired_vel_change / time.delta_secs();
        let force = steering_dir * desired_accel;
        forces.apply_linear_impulse_at_point(force, global_transform.translation());
    }
}
