use crate::car::{
    horn::HornPlugin, inputs::CarRemoteInputsPlugin, steering_wheel::SteeringWheelPlugin,
};
use bevy::prelude::*;

mod horn;
mod inputs;
pub mod spawn;
pub mod steering_wheel;

pub struct GameCarPlugin;
impl Plugin for GameCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarRemoteInputsPlugin, SteeringWheelPlugin, HornPlugin));
    }
}
