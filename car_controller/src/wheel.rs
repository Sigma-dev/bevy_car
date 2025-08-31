use avian3d::prelude::*;
use bevy::prelude::*;

pub struct CarWheelPlugin;

impl Plugin for CarWheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, handle_power);
    }
}

#[derive(Component)]
pub struct CarWheel {
    power: f32,
}

impl CarWheel {
    pub fn new(power: f32) -> Self {
        Self { power }
    }
}

fn handle_power(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    wheels: Query<(&GlobalTransform, &Transform, &CarWheel, &ChildOf)>,
    mut parents: Query<Option<&mut ExternalForce>>,
) {
    let input = if keyboard.pressed(KeyCode::KeyW) {
        1.0
    } else if keyboard.pressed(KeyCode::KeyS) {
        -1.0
    } else {
        return;
    };
    for (global_transform, transform, wheel, suspension) in wheels.iter() {
        let external_force = parents.get_mut(suspension.0).unwrap();
        let force = transform.forward() * wheel.power * input;
        println!("force: {}", force);

        if let Some(mut external_force) = external_force {
            external_force.apply_force_at_point(force, global_transform.translation(), Vec3::ZERO);
            external_force.persistent = false;
        } else {
            commands
                .entity(suspension.0)
                .insert(ExternalForce::new(Vec3::Y * force).with_persistence(false));
        }
    }
}

fn _get_point_velocity(linear_velocity: Vec3, angular_velocity: Vec3, point: Vec3) -> Vec3 {
    linear_velocity + angular_velocity.cross(point)
}
