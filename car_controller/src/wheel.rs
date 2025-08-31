use avian3d::prelude::*;
use bevy::prelude::*;
pub struct CarWheelPlugin;

impl Plugin for CarWheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (handle_wheels_added, handle_spring).chain());
    }
}

#[derive(Component)]
pub struct CarWheel {
    pub(crate) origin_offset: f32,
    pub(crate) radius: f32,
    pub(crate) spring_stiffness: f32,
    pub(crate) damping_ratio: f32,
    pub(crate) spring_velocity: f32,
}

impl CarWheel {
    pub fn new(origin_offset: f32, radius: f32, spring_stiffness: f32, damping_ratio: f32) -> Self {
        Self {
            origin_offset,
            radius,
            spring_stiffness,
            damping_ratio,
            spring_velocity: 0.0,
        }
    }
}

fn handle_wheels_added(
    mut commands: Commands,
    mut wheel: Query<(Entity, &mut CarWheel, &Transform), Added<CarWheel>>,
) {
    for (entity, mut wheel, transform) in wheel.iter_mut() {
        commands
            .entity(entity)
            .insert(RayCaster::new(Vec3::Y * wheel.origin_offset, Dir3::NEG_Y).with_max_hits(1));
    }
}

fn handle_spring(
    time: Res<Time>,
    mut commands: Commands,
    mut car_wheels: Query<(
        &mut CarWheel,
        &mut Transform,
        &GlobalTransform,
        &RayHits,
        &ChildOf,
        &Name,
    )>,
    mut external_forces: Query<&mut ExternalForce>,
    velocities: Query<(&LinearVelocity, &AngularVelocity)>,
) {
    for (mut wheel, mut transform, global_transform, ray_hits, parent, name) in
        car_wheels.iter_mut()
    {
        let Some(hit) = ray_hits.iter().next() else {
            println!("{}: no hit", name);
            continue;
        };
        println!("{}: distance: {}", name, hit.distance);
        let offset = wheel.origin_offset - hit.distance;

        /*  println!(
            "ray_position: {}, current_height: {}",
            ray_position, transform.translation.y
        ); */

        let (linear_velocity, angular_velocity) = velocities.get(parent.0).unwrap();
        let wheel_velocity = get_point_velocity(
            linear_velocity.0,
            angular_velocity.0,
            global_transform.translation(),
        );
        let force: Vec3 =
            (offset * wheel.spring_stiffness) - (wheel_velocity * wheel.damping_ratio);
        println!(
            "{}: force: {}, offset: {}, wheel_velocity: {}",
            name, force, offset, wheel_velocity
        );

        if let Ok(mut external_force) = external_forces.get_mut(parent.0) {
            external_force.apply_force_at_point(
                Vec3::Y * force,
                global_transform.translation(),
                Vec3::ZERO,
            );
        } else {
            commands
                .entity(parent.0)
                .insert(ExternalForce::new(Vec3::Y * force));
        }
    }
}

fn get_point_velocity(linear_velocity: Vec3, angular_velocity: Vec3, point: Vec3) -> Vec3 {
    linear_velocity + angular_velocity.cross(point)
}
