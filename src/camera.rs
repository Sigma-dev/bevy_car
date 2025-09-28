use bevy::{core_pipeline::bloom::Bloom, pbr::Atmosphere, prelude::*, render::camera::Exposure};

#[derive(Bundle)]
pub struct CameraBundle {
    camera3d: Camera3d,
    camera: Camera,
    atmosphere: Atmosphere,
    exposure: Exposure,
    bloom: Bloom,
}

impl CameraBundle {
    pub fn new() -> Self {
        Self {
            camera3d: Camera3d::default(),
            camera: Camera {
                hdr: true,
                ..default()
            },
            atmosphere: Atmosphere::EARTH,
            exposure: Exposure::OVERCAST,
            bloom: Bloom::NATURAL,
        }
    }
}
