use avian3d::prelude::*;
use bevy::prelude::*;

use crate::vertical_suspension::VerticalSuspensionCurrentLength;

pub struct CarWheelVisualsPlugin;

impl Plugin for CarWheelVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_visuals);
    }
}

#[derive(Component)]
pub struct CarWheelVisuals {
    radius: f32,
}

impl CarWheelVisuals {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

fn handle_visuals(
    time: Res<Time>,
    children_of: Query<&ChildOf>,
    rigid_bodies: Query<(&GlobalTransform, &LinearVelocity)>,
    mut wheels: Query<(Entity, &mut Transform, &ChildOf, &CarWheelVisuals)>,
    suspensions: Query<(&VerticalSuspensionCurrentLength)>,
) {
    for (entity, mut transform, child_of, visuals) in wheels.iter_mut() {
        let Ok(suspension) = suspensions.get(child_of.0) else {
            continue;
        };
        transform.translation.y = -suspension.0 + visuals.radius;
        let rigidbody = children_of
            .iter_ancestors(entity)
            .find(|a| rigid_bodies.contains(*a))
            .unwrap();
        let (rigidbody_transform, velocity) = rigid_bodies.get(rigidbody).unwrap();
        let forward_velocity = velocity.0.length() * velocity.0.dot(*rigidbody_transform.forward());
        transform.rotation *= Quat::from_rotation_x(-forward_velocity * time.delta_secs() * 1.);
    }
}
