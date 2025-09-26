use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_steam_p2p::SteamP2PPlugin;
use car_controller::prelude::*;
use fps_camera::prelude::*;
use numpad_cameras::prelude::*;

use crate::{car::GameCarPlugin, lobby::LobbyPlugin};

pub mod car;
pub mod lobby;
pub mod world;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_plugins((
            SteamP2PPlugin,
            NumpadCamerasPlugin,
            FpsCameraPlugin,
            CarControllerPlugin,
            CarControllerDebugPlugin,
        ))
        .add_plugins((LobbyPlugin, GameCarPlugin))
        .run()
}
