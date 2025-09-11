use avian3d::prelude::*;
use bevy::prelude::*;

pub mod prelude;

pub struct ForceAccumulatorPlugin;

impl Plugin for ForceAccumulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, update_force_accumulator);
    }
}

#[derive(Component)]
pub struct ForceAccumulator {
    forces: Vec<ForceAccumulatorEntry>,
}

#[derive(Debug)]
struct ForceAccumulatorEntry {
    force: Vec3,
    point: Vec3,
    center_of_mass: Vec3,
    is_impulse: bool,
    debug_color: Option<Color>,
}

impl ForceAccumulator {
    pub fn new() -> Self {
        Self { forces: Vec::new() }
    }

    pub fn apply_force(&mut self, force: Vec3, point: Vec3, center_of_mass: Vec3) {
        self.forces.push(ForceAccumulatorEntry {
            force,
            point,
            center_of_mass,
            is_impulse: false,
            debug_color: None,
        });
    }

    pub fn apply_impulse(&mut self, force: Vec3, point: Vec3, center_of_mass: Vec3) {
        self.forces.push(ForceAccumulatorEntry {
            force,
            point,
            center_of_mass,
            is_impulse: true,
            debug_color: None,
        });
    }

    pub fn apply_force_debug(
        &mut self,
        force: Vec3,
        point: Vec3,
        center_of_mass: Vec3,
        color: Color,
    ) {
        self.forces.push(ForceAccumulatorEntry {
            force,
            point,
            center_of_mass,
            is_impulse: false,
            debug_color: Some(color),
        });
    }

    pub fn apply_impulse_debug(
        &mut self,
        force: Vec3,
        point: Vec3,
        center_of_mass: Vec3,
        color: Color,
    ) {
        self.forces.push(ForceAccumulatorEntry {
            force,
            point,
            center_of_mass,
            is_impulse: true,
            debug_color: Some(color),
        });
    }
}

fn update_force_accumulator(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    mut gizmos: Gizmos,
    mut force_accumulators: Query<(
        Entity,
        &ComputedMass,
        &mut ForceAccumulator,
        Option<&mut ExternalForce>,
    )>,
) {
    for (entity, _mass, mut force_accumulator, maybe_external_force) in
        force_accumulators.iter_mut()
    {
        let mut external_force = maybe_external_force
            .map(|e| *e)
            .unwrap_or(ExternalForce::new(Vec3::ZERO));
        for force in &mut force_accumulator.forces {
            let computed_force = if force.is_impulse {
                force.force / time.delta_secs()
            } else {
                force.force
            };
            external_force.apply_force_at_point(computed_force, force.point, force.center_of_mass);
            if let Some(color) = force.debug_color {
                gizmos.arrow(force.point, force.point + force.force, color);
            }
        }
        external_force.persistent = false;
        force_accumulator.forces.clear();
        commands.entity(entity).insert(external_force);
    }
}
