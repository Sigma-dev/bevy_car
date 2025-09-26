use bevy::prelude::*;
use car_controller::CarController;

pub struct SteeringWheelPlugin;
impl Plugin for SteeringWheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_steering_wheel);
    }
}

#[derive(Component)]
pub struct SteeringWheel {
    pub rotation_multiplier: f32,
}

fn update_steering_wheel(
    mut steering_wheel: Query<(Entity, &mut Transform, &SteeringWheel)>,
    car: Query<&CarController>,
    children_of: Query<&ChildOf>,
) {
    for (entity, mut transform, steering_wheel) in steering_wheel.iter_mut() {
        let car_entity = children_of
            .iter_ancestors(entity)
            .find(|a| car.contains(*a))
            .unwrap();
        let car_controller = car.get(car_entity).unwrap();
        transform.rotation =
            Quat::from_rotation_y(car_controller.steer_angle * steering_wheel.rotation_multiplier);
    }
}
