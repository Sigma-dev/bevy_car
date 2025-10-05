use bevy::{camera::Exposure, pbr::Atmosphere, post_process::bloom::Bloom, prelude::*};

#[derive(Bundle)]
pub struct CameraBundle {
    camera3d: Camera3d,
    atmosphere: Atmosphere,
    exposure: Exposure,
    bloom: Bloom,
}

impl CameraBundle {
    pub fn new() -> Self {
        Self {
            camera3d: Camera3d::default(),
            atmosphere: Atmosphere::EARTH,
            exposure: Exposure::OVERCAST,
            bloom: Bloom::NATURAL,
        }
    }
}
