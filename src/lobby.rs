use bevy::prelude::*;
use bevy_steam_p2p::prelude::*;

use crate::{car::spawn_car, world::spawn_world};

pub struct LobbyPlugin;
impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (menu, on_lobby_join, handle_unhandled_instantiations),
        );
    }
}

fn menu(client: ResMut<SteamP2PClient>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyC) {
        client.create_lobby(8);
    }
}

fn on_lobby_join(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut join_r: EventReader<LobbyJoined>,
    mut client: ResMut<SteamP2PClient>,
) {
    if !join_r.is_empty() {
        join_r.clear();
        spawn_world(&mut commands, &asset_server);
        if client.is_lobby_owner().unwrap() {
            client
                .instantiate(FilePath::new("Player"), None, Transform::default())
                .expect("Couldn't spawn player");
        }
    }
}

fn handle_unhandled_instantiations(
    mut commands: Commands,
    mut evs_unhandled: EventReader<UnhandledInstantiation>,
    asset_server: ResMut<AssetServer>,
    client: ResMut<SteamP2PClient>,
) {
    for UnhandledInstantiation(data) in evs_unhandled.read() {
        match data.network_identity.instantiation_path.0.as_str() {
            "Player" => {
                println!("Instantiated Player");
                spawn_car(
                    &mut commands,
                    &asset_server,
                    data.starting_transform,
                    data.network_identity.clone(),
                );
            }
            _ => {
                println!("No valid instantiation candidate found");
            }
        }
    }
}
