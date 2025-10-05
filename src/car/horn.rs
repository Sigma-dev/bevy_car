use audio_manager::prelude::*;
use bevy::prelude::*;
use bevy_steam_p2p::prelude::{Networked, NetworkedMessages};
use serde::{Deserialize, Serialize};

pub struct HornPlugin;
impl Plugin for HornPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_horn, handle_horn_sound))
            .add_networked_message::<HornSound>();
    }
}

#[derive(Component)]
pub struct Horn;

#[derive(Message, Serialize, Deserialize, Clone)]
pub struct HornSound(pub Vec3);

fn handle_horn_sound(mut audio_manager: AudioManager, mut horn_sound: MessageReader<HornSound>) {
    for HornSound(position) in horn_sound.read() {
        audio_manager.play_sound(
            PlayAudio3D::new_once("sounds/car/horn.wav").with_spatial(Some((*position, None))),
        );
    }
}

fn handle_horn(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut horn: Query<&Transform, With<Horn>>,
    mut horn_sound: MessageWriter<Networked<HornSound>>,
) {
    for transform in horn.iter_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            horn_sound.write(Networked::new(HornSound(transform.translation)));
        }
    }
}
