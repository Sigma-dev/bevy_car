use bevy::prelude::*;
use bevy_steam_p2p::prelude::*;
use car_controller::prelude::*;
use serde::{Deserialize, Serialize};

use crate::lobby::{CurrentGameState, GameState};

pub struct CarRemoteInputsPlugin;
impl Plugin for CarRemoteInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (emit_inputs, receive_inputs))
            .add_networked_message::<CarRemoteInputs>();
    }
}

#[derive(Message, Serialize, Deserialize, Clone)]
pub struct CarRemoteInputs(SteamId, CarControllerInputs);

#[derive(Component)]
pub struct RemotelyControlled(pub SteamId);

fn emit_inputs(
    client: ResMut<SteamP2PClient>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut remote_inputs: MessageWriter<Networked<CarRemoteInputs>>,
) {
    if client.is_lobby_owner().unwrap_or(false) {
        return;
    }

    let inputs = CarControllerInputs::from_keyboard(&keyboard);
    remote_inputs.write(Networked::new(CarRemoteInputs(client.id, inputs)));
}

fn receive_inputs(
    client: Res<SteamP2PClient>,
    current_game_state: Res<CurrentGameState>,
    mut remote_inputs: MessageReader<CarRemoteInputs>,
    mut car_controller_input: Query<(&RemotelyControlled, &mut CarControllerInput)>,
) {
    if !client.is_lobby_owner().unwrap_or(false) {
        return;
    }
    if current_game_state.0 != GameState::Race {
        return;
    }
    for CarRemoteInputs(steam_id, inputs) in remote_inputs.read() {
        for (car_identity, mut car_controller_input) in car_controller_input.iter_mut() {
            if car_identity.0 == *steam_id {
                car_controller_input.update(*inputs);
            }
        }
    }
}
