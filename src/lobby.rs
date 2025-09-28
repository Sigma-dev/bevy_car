use bevy::prelude::*;
use bevy_steam_p2p::prelude::*;
use numpad_cameras::prelude::*;

use crate::{car::spawn::spawn_car, world::spawn_world};

pub struct LobbyPlugin;
impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                menu,
                on_lobby_join,
                on_other_joined,
                handle_unhandled_instantiations,
            ),
        );
    }
}

fn menu(client: ResMut<SteamP2PClient>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyC) {
        client.create_lobby(8);
    }
}

const POSITION: Vec3 = Vec3::new(-7., 0.5, 5.);

fn on_lobby_join(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut join_r: EventReader<LobbyJoined>,
    mut client: ResMut<SteamP2PClient>,
    mut current_numpad_camera: ResMut<CurrentNumpadCamera>,
) {
    if !join_r.is_empty() {
        join_r.clear();
        spawn_world(&mut commands, &asset_server);
        current_numpad_camera.set(KeyCode::Numpad4);
        if client.is_lobby_owner().unwrap() {
            client
                .instantiate(
                    FilePath::new("Player_owner"),
                    None,
                    Transform::from_translation(POSITION),
                )
                .expect("Couldn't spawn player");
        }
    }
}

fn on_other_joined(
    mut other_joined_r: EventReader<OtherJoined>,
    mut client: ResMut<SteamP2PClient>,
) {
    for OtherJoined(other_joined) in other_joined_r.read() {
        let count = client.get_lobby_member_count().unwrap();
        if client.is_lobby_owner().unwrap() {
            client
                .instantiate(
                    FilePath::new(&other_joined.raw().to_string()),
                    None,
                    Transform::from_translation(
                        POSITION + Vec3::new((count - 1) as f32 * 4., 0., 0.),
                    ),
                )
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
        let is_lobby_owner = client.is_lobby_owner().unwrap();
        match data.network_identity.instantiation_path.0.as_str() {
            "Player_owner" => {
                spawn_car(
                    &mut commands,
                    &asset_server,
                    data.starting_transform,
                    data.network_identity.clone(),
                    None,
                    is_lobby_owner,
                    is_lobby_owner,
                );
            }
            steam_id => {
                let steam_id = SteamId::from_raw(steam_id.parse().unwrap());
                spawn_car(
                    &mut commands,
                    &asset_server,
                    data.starting_transform,
                    data.network_identity.clone(),
                    Some(steam_id),
                    is_lobby_owner,
                    steam_id == client.id,
                );
            }
        }
    }
}
