use bevy::prelude::*;

use crate::wheel::CarWheel;

pub struct CarControllerDebugPlugin;

impl Plugin for CarControllerDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_wheels);
    }
}

fn debug_wheels(mut gizmos: Gizmos, car_wheels: Query<(&GlobalTransform, &CarWheel)>) {
    for (transform, wheel) in car_wheels.iter() {
        let origin_offset_global = transform.translation() + Vec3::Y * wheel.origin_offset;

        gizmos.line(origin_offset_global, transform.translation(), Color::WHITE);
        gizmos.sphere(
            transform.translation(),
            wheel.radius,
            Color::linear_rgb(1.00, 0.00, 0.00),
        );
    }
}
