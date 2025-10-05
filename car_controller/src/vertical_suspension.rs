use avian3d::prelude::*;
use bevy::prelude::*;

pub struct VerticalSuspensionPlugin;

impl Plugin for VerticalSuspensionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, on_suspension_added);
        app.add_systems(FixedUpdate, handle_vertical_suspension);
    }
}

#[derive(Component)]
pub struct VerticalSuspension {
    pub stiffness: f32,
    pub damping_ratio: f32,
    pub travel_distance: f32,
}

#[derive(Component)]
pub struct VerticalSuspensionCurrentLength(pub f32);

impl VerticalSuspension {
    pub fn new(stiffness: f32, damping_ratio: f32, travel_distance: f32) -> Self {
        Self {
            stiffness,
            damping_ratio,
            travel_distance,
        }
    }
}

fn on_suspension_added(
    mut commands: Commands,
    suspension: Query<(Entity, &VerticalSuspension), Added<VerticalSuspension>>,
) {
    for (entity, vertical_suspension) in suspension.iter() {
        commands
            .entity(entity)
            .insert((RayCaster::new(Vec3::ZERO, Dir3::NEG_Y)
                .with_max_distance(vertical_suspension.travel_distance),));
    }
}

fn handle_vertical_suspension(
    mut commands: Commands,
    vertical_suspensions: Query<(
        Entity,
        &GlobalTransform,
        &VerticalSuspension,
        &ChildOf,
        &RayHits,
    )>,
    mut parent: Query<Forces>,
) {
    for (entity, global_transform, vertical_suspension, child_of, hits) in
        vertical_suspensions.iter()
    {
        let Some(hit_distance) = hits.iter().next().map(|h| h.distance) else {
            continue;
        };
        let Ok(mut forces) = parent.get_mut(child_of.0) else {
            continue;
        };
        commands
            .entity(entity)
            .insert(VerticalSuspensionCurrentLength(hit_distance));
        let offset = vertical_suspension.travel_distance - hit_distance;

        let velocity = forces.velocity_at_point(global_transform.translation());
        let y_velocity = velocity.dot(*global_transform.up()) * global_transform.up();
        let force = (global_transform.up() * offset * vertical_suspension.stiffness)
            - (y_velocity * vertical_suspension.damping_ratio);

        forces.apply_linear_impulse_at_point(force, global_transform.translation());
    }
}
