use avian3d::prelude::*;
use bevy::prelude::*;

use crate::wheel::CarWheel;

pub struct CarControllerDebugPlugin;

impl Plugin for CarControllerDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_debug);
    }
}

fn handle_debug(
    mut gizmos: Gizmos,
    wheels: Query<(&GlobalTransform, &ChildOf), With<CarWheel>>,
    parents: Query<(&GlobalTransform, &mut LinearVelocity, &mut AngularVelocity)>,
) {
    for (global_transform, child_of) in wheels.iter() {
        let (parent_global_transform, linear_velocity, angular_velocity) =
            parents.get(child_of.0).unwrap();
        let velocity = get_point_velocity(
            linear_velocity.0,
            angular_velocity.0,
            global_transform.translation() - parent_global_transform.translation(),
        );
        gizmos.arrow(
            global_transform.translation(),
            global_transform.translation() + velocity * 10.,
            Color::srgb(0.77, 0.02, 0.87),
        );
    }
}

fn get_point_velocity(linear_velocity: Vec3, angular_velocity: Vec3, point: Vec3) -> Vec3 {
    linear_velocity + angular_velocity.cross(point)
}
