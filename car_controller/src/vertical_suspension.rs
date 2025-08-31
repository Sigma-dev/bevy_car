use avian3d::prelude::*;
use bevy::prelude::*;

pub struct VerticalSuspensionPlugin;

impl Plugin for VerticalSuspensionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, on_suspension_added);
        app.add_systems(FixedUpdate, (handle_vertical_suspension,));
    }
}

#[derive(Component)]
#[relationship(relationship_target = AttachedSuspensions)]
pub struct Suspension(pub Entity);

/// This is the "relationship target" component.
/// It will be automatically inserted and updated to contain
/// all entities that currently "like" this entity.
#[derive(Component, Deref)]
#[relationship_target(relationship = Suspension)]
pub struct AttachedSuspensions(Vec<Entity>);

#[derive(Component)]
pub struct VerticalSuspension {
    pub stiffness: f32,
    pub damping_ratio: f32,
    diff: f32,
}

impl VerticalSuspension {
    pub fn new(stiffness: f32, damping_ratio: f32) -> Self {
        Self {
            stiffness,
            damping_ratio,
            diff: 0.0,
        }
    }
}

fn on_suspension_added(
    mut suspension: Query<(&Transform, &mut VerticalSuspension, &Suspension), Added<Suspension>>,
    transforms: Query<&Transform>,
) {
    for (transform, mut vertical_suspension, suspension) in suspension.iter_mut() {
        let parent_transform = transforms.get(suspension.0).unwrap();
        vertical_suspension.diff = parent_transform.translation.y - transform.translation.y;
    }
}

fn handle_vertical_suspension(
    mut commands: Commands,
    vertical_suspensions: Query<(
        &GlobalTransform,
        &Transform,
        &VerticalSuspension,
        &Suspension,
    )>,
    mut parent: Query<(
        &Transform,
        &LinearVelocity,
        &AngularVelocity,
        Option<&mut ExternalForce>,
    )>,
) {
    for (global_transform, transform, vertical_suspension, suspension) in
        vertical_suspensions.iter()
    {
        let (parent_transform, linear_velocity, angular_velocity, external_force) =
            parent.get_mut(suspension.0).unwrap();
        let diff = parent_transform.translation.y - transform.translation.y;
        let offset = vertical_suspension.diff - diff;
        let velocity =
            get_point_velocity(linear_velocity.0, angular_velocity.0, transform.translation);
        let force = (Vec3::Y * offset * vertical_suspension.stiffness)
            - (velocity * vertical_suspension.damping_ratio);

        if let Some(mut external_force) = external_force {
            external_force.apply_force_at_point(
                Vec3::Y * force,
                global_transform.translation(),
                Vec3::ZERO,
            );
            external_force.persistent = false;
        } else {
            commands
                .entity(suspension.0)
                .insert(ExternalForce::new(Vec3::Y * force).with_persistence(false));
        }
    }
}

fn get_point_velocity(linear_velocity: Vec3, angular_velocity: Vec3, point: Vec3) -> Vec3 {
    linear_velocity + angular_velocity.cross(point)
}
