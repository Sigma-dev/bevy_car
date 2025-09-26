use bevy::prelude::*;

use crate::PlayAudio;

#[derive(Clone)]
pub struct PlayAudio3D {
    pub path: String,
    pub volume_mult: f32,
    pub one_shot: bool,
    pub spatial_settings: Option<(Vec3, Option<Entity>)>,
}

impl PlayAudio3D {
    pub fn new_once(path: impl Into<String>) -> PlayAudio3D {
        PlayAudio3D {
            path: path.into(),
            volume_mult: 1.,
            one_shot: true,
            spatial_settings: None,
        }
    }

    pub fn new_repeating(path: impl Into<String>) -> PlayAudio3D {
        PlayAudio3D {
            path: path.into(),
            volume_mult: 1.,
            one_shot: false,
            spatial_settings: None,
        }
    }

    pub fn with_volume(&self, volume_mult: f32) -> PlayAudio3D {
        let mut new = self.clone();
        new.volume_mult = volume_mult;
        new
    }

    pub fn with_spatial(&self, spatial_settings: Option<(Vec3, Option<Entity>)>) -> PlayAudio3D {
        let mut new = self.clone();
        new.spatial_settings = spatial_settings;
        new
    }
}

impl PlayAudio for PlayAudio3D {
    fn is_one_shot(&self) -> bool {
        self.one_shot
    }
    fn volume_mult(&self) -> f32 {
        self.volume_mult
    }
    fn path(&self) -> String {
        self.path.clone()
    }
    fn get_spatial(&self) -> Option<(Transform, Option<Entity>)> {
        self.spatial_settings
            .clone()
            .map(|(position, maybe_follow)| (Transform::from_translation(position), maybe_follow))
    }
}
