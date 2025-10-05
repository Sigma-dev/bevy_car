use avian3d::prelude::*;
use bevy::prelude::*;

use crate::wheels::CarWheel;

pub struct CarControllerDebugPlugin;

impl Plugin for CarControllerDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_debug);
    }
}

fn handle_debug(
    mut gizmos: Gizmos,
    wheels: Query<(&GlobalTransform, &ChildOf), With<CarWheel>>,
    mut cars: Query<Forces>,
) {
    for (global_transform, child_of) in wheels.iter() {
        let forces = cars.get_mut(child_of.0).unwrap();
        let velocity = forces.velocity_at_point(global_transform.translation());
        gizmos.arrow(
            global_transform.translation(),
            global_transform.translation() + velocity * 10.,
            Color::srgb(0.77, 0.02, 0.87),
        );
        gizmos.arrow(
            global_transform.translation(),
            global_transform.translation() + *global_transform.forward() * 10.,
            Color::srgb(1.00, 0.32, 0.00),
        );
    }
}
