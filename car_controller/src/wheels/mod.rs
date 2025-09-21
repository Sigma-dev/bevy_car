use avian3d::prelude::*;
use bevy::prelude::*;
use force_accumulator::ForceAccumulator;

use crate::{inputs::CarControllerInput, wheels::visuals::CarWheelVisualsPlugin};

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
    power: f32,
    grip: f32,
    rolling_resistance: f32,
    can_turn: bool,
}

impl CarWheel {
    pub fn new(power: f32, grip: f32, rolling_resistance: f32, can_turn: bool) -> Self {
        Self {
            power,
            grip,
            rolling_resistance,
            can_turn,
        }
    }
}

fn handle_rolling_resistance(
    wheels: Query<(&GlobalTransform, &CarWheel, &ChildOf)>,
    mut parents: Query<(
        &GlobalTransform,
        &LinearVelocity,
        &AngularVelocity,
        &mut ForceAccumulator,
    )>,
) {
    {
        for (global_transform, wheel, suspension) in wheels.iter() {
            let Ok((
                parent_global_transform,
                linear_velocity,
                angular_velocity,
                mut force_accumulator,
            )) = parents.get_mut(suspension.0)
            else {
                continue;
            };
            let velocity = get_point_velocity(
                linear_velocity.0,
                angular_velocity.0,
                global_transform.translation() - parent_global_transform.translation(),
            );
            let rolling_resistance_force = -velocity * wheel.rolling_resistance;

            force_accumulator.apply_impulse(
                rolling_resistance_force,
                global_transform.translation(),
                parent_global_transform.translation(),
            );
        }
    }
}

fn handle_turning(
    mut wheels: Query<(&mut Transform, &CarWheel, &ChildOf)>,
    parents: Query<&CarControllerInput>,
) {
    for (mut transform, wheel, child_of) in wheels.iter_mut() {
        if !wheel.can_turn {
            continue;
        }
        let input = parents.get(child_of.0).unwrap();
        let input = if input.left {
            1.0
        } else if input.right {
            -1.0
        } else {
            continue;
        };
        let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let max_angle = 30_f32.to_radians();
        let new_yaw = (yaw + input * 0.05).clamp(-max_angle, max_angle);
        transform.rotation = Quat::from_rotation_y(new_yaw);
    }
}

fn handle_power(
    mut gizmos: Gizmos,
    wheels: Query<(&GlobalTransform, &CarWheel, &ChildOf)>,
    mut parents: Query<(&GlobalTransform, &mut ForceAccumulator, &CarControllerInput)>,
) {
    for (global_transform, wheel, suspension) in wheels.iter() {
        if wheel.power == 0.0 {
            continue;
        }
        let Ok((parent_global_transform, mut force_accumulator, car_controller_input)) =
            parents.get_mut(suspension.0)
        else {
            continue;
        };
        gizmos.arrow(
            global_transform.translation(),
            global_transform.translation() + *global_transform.forward() * 10.,
            Color::srgb(1.00, 0.32, 0.00),
        );
        let input = if car_controller_input.forward {
            1.0
        } else if car_controller_input.backward {
            -1.0
        } else {
            continue;
        };

        let force = global_transform.forward() * wheel.power * input;

        force_accumulator.apply_impulse_debug(
            force,
            global_transform.translation(),
            parent_global_transform.translation(),
            Color::srgb(1.0, 0.0, 0.0),
        );
    }
}

fn handle_traction(
    time: Res<Time>,
    wheels: Query<(&GlobalTransform, &CarWheel, &ChildOf)>,
    mut parents: Query<(
        &GlobalTransform,
        &LinearVelocity,
        &AngularVelocity,
        &mut ForceAccumulator,
    )>,
) {
    for (global_transform, wheel, suspension) in wheels.iter() {
        let Ok((parent_global_transform, linear_velocity, angular_velocity, mut force_accumulator)) =
            parents.get_mut(suspension.0)
        else {
            continue;
        };
        let steering_dir = global_transform.right().as_vec3();
        let velocity = get_point_velocity(
            linear_velocity.0,
            angular_velocity.0,
            global_transform.translation() - parent_global_transform.translation(),
        );
        let steering_vel = steering_dir.dot(velocity);
        let desired_vel_change = -steering_vel * wheel.grip;
        let desired_accel = desired_vel_change / time.delta_secs();
        let force = steering_dir * desired_accel;
        force_accumulator.apply_force_debug(
            force,
            global_transform.translation(),
            parent_global_transform.translation(),
            Color::srgb(0.0, 0.0, 1.0),
        );
    }
}

fn get_point_velocity(linear_velocity: Vec3, angular_velocity: Vec3, point: Vec3) -> Vec3 {
    linear_velocity + angular_velocity.cross(point)
}
