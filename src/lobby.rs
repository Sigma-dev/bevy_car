use audio_manager::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_steam_p2p::prelude::*;
use car_controller::prelude::*;
use numpad_cameras::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{car::spawn::spawn_car, world::spawn_world};

pub struct LobbyPlugin;
impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_street_light_material)
            .add_systems(
                Update,
                (
                    start_game,
                    menu,
                    on_race_started,
                    on_lobby_join,
                    on_other_joined,
                    handle_unhandled_instantiations,
                    race_light,
                ),
            )
            .add_systems(PostUpdate, waiting)
            .insert_resource(CurrentGameState(GameState::Waiting))
            .add_networked_message::<RaceStarted>();
    }
}

#[derive(Resource)]
pub struct CurrentGameState(pub GameState);

#[derive(PartialEq)]
pub enum GameState {
    Waiting,
    Countdown(f32),
    Race,
}

#[derive(Message, Serialize, Deserialize, Clone, Copy)]
pub struct RaceStarted;

#[derive(Resource)]
pub struct RaceLightMaterial(Handle<StandardMaterial>);

fn make_emissive(color: Color) -> StandardMaterial {
    let mut material = StandardMaterial::from_color(color);
    material.emissive = LinearRgba::from(color) * 1000.;
    material
}

fn initialize_street_light_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(RaceLightMaterial(
        materials.add(StandardMaterial::from_color(Color::srgb(1., 0., 0.))),
    ));
}

fn race_light(
    time: Res<Time>,
    mut current_game_state: ResMut<CurrentGameState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    race_light_material: Res<RaceLightMaterial>,
) {
    if let GameState::Countdown(elapsed) = current_game_state.0 {
        let elapsed = time.elapsed_secs() - elapsed;

        let material = if elapsed < 1. {
            make_emissive(Color::srgb(0.53, 0.15, 0.00))
        } else if elapsed < 2. {
            make_emissive(Color::srgb(1., 0., 0.))
        } else if elapsed < 3. {
            make_emissive(Color::srgb(1.00, 0.57, 0.00))
        } else {
            make_emissive(Color::srgb(0.00, 1.00, 0.22))
        };
        let _ = materials.insert(race_light_material.0.id(), material);

        if elapsed > 3. {
            current_game_state.0 = GameState::Race;
        }
    }
}

fn waiting(
    current_game_state: Res<CurrentGameState>,
    mut cars: Query<&mut LinearVelocity, With<CarController>>,
) {
    if current_game_state.0 == GameState::Race {
        return;
    }
    for mut linear_velocity in cars.iter_mut() {
        linear_velocity.0 = Vec3::ZERO;
    }
}

fn start_game(
    mut audio_manager: AudioManager,
    client: ResMut<SteamP2PClient>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut evs_race_started: MessageWriter<Networked<RaceStarted>>,
    current_game_state: Res<CurrentGameState>,
) {
    let Ok(is_lobby_owner) = client.is_lobby_owner() else {
        return;
    };
    if current_game_state.0 != GameState::Waiting {
        return;
    }
    if is_lobby_owner && keyboard_input.just_pressed(KeyCode::KeyP) {
        evs_race_started.write(Networked::new(RaceStarted));
        audio_manager.play_sound(PlayAudio3D::new_once("sounds/world/countdown.wav"));
    }
}

fn on_race_started(
    time: Res<Time>,
    client: ResMut<SteamP2PClient>,
    mut current_game_state: ResMut<CurrentGameState>,
    mut evs_race_started: MessageReader<RaceStarted>,
) {
    for RaceStarted in evs_race_started.read() {
        if current_game_state.0 != GameState::Waiting {
            return;
        }
        current_game_state.0 = GameState::Countdown(time.elapsed_secs());
        if !client.is_lobby_owner().unwrap_or(false) {
            return;
        }
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
    mut join_r: MessageReader<LobbyJoined>,
    mut client: ResMut<SteamP2PClient>,
    mut current_numpad_camera: ResMut<CurrentNumpadCamera>,
    race_light_material: Res<RaceLightMaterial>,
) {
    if !join_r.is_empty() {
        join_r.clear();
        spawn_world(&mut commands, &asset_server, race_light_material.0.clone());
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
    mut other_joined_r: MessageReader<OtherJoined>,
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
    mut evs_unhandled: MessageReader<UnhandledInstantiation>,
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
