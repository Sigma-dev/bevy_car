use avian3d::prelude::*;
use bevy::prelude::*;
use force_accumulator::prelude::*;

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
    vertical_suspensions: Query<(&GlobalTransform, &VerticalSuspension, &ChildOf, &RayHits)>,
    mut parent: Query<(
        &GlobalTransform,
        &LinearVelocity,
        &AngularVelocity,
        &mut ForceAccumulator,
    )>,
) {
    for (global_transform, vertical_suspension, suspension, hits) in vertical_suspensions.iter() {
        let Some(hit_distance) = hits.iter().next().map(|h| h.distance) else {
            continue;
        };
        let (parent_global_transform, linear_velocity, angular_velocity, mut force_accumulator) =
            parent.get_mut(suspension.0).unwrap();
        let offset = vertical_suspension.travel_distance - hit_distance;

        let velocity = get_point_velocity(
            linear_velocity.0,
            angular_velocity.0,
            global_transform.translation() - parent_global_transform.translation(),
        );
        let y_velocity = velocity.dot(*global_transform.up()) * global_transform.up();
        let force = (global_transform.up() * offset * vertical_suspension.stiffness)
            - (y_velocity * vertical_suspension.damping_ratio);

        force_accumulator.apply_impulse_debug(
            force,
            global_transform.translation(),
            parent_global_transform.translation(),
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
}

fn get_point_velocity(linear_velocity: Vec3, angular_velocity: Vec3, point: Vec3) -> Vec3 {
    linear_velocity + angular_velocity.cross(point)
}
