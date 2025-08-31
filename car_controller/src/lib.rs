use bevy::prelude::*;

use crate::{vertical_suspension::VerticalSuspensionPlugin, wheel::CarWheelPlugin};

pub mod debug;
pub mod prelude;
mod vertical_suspension;
mod wheel;
pub struct CarControllerPlugin;

impl Plugin for CarControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarWheelPlugin, VerticalSuspensionPlugin));
    }
}
