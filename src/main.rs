use audio_manager::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_steam_p2p::prelude::*;
use car_controller::prelude::*;
use fps_camera::prelude::*;
use numpad_cameras::prelude::*;
use replace_material::prelude::*;

use crate::{car::GameCarPlugin, lobby::LobbyPlugin};

mod camera;
pub mod car;
pub mod lobby;
pub mod world;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins((
            SteamP2PPlugin,
            NumpadCamerasPlugin,
            FpsCameraPlugin,
            CarControllerPlugin,
            AudioManagerPlugin::default(),
            ReplaceMaterialPlugin,
        ))
        .add_plugins((LobbyPlugin, GameCarPlugin))
        .run()
}
