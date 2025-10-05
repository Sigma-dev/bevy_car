use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

pub mod prelude {
    pub use crate::{FpsCamera, FpsCameraPlugin};
}

pub struct FpsCameraPlugin;

impl Plugin for FpsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_fps_cameras)
            .add_systems(Update, setup);
    }
}

#[derive(Component)]
#[require(Camera3d)]
pub struct FpsCamera {
    pub sensitivity: f32,
    pub rotate_parent_y: bool,
}

#[derive(Component, Debug)]
pub struct ShowCursor;

impl FpsCamera {
    pub fn new(sensitivity: f32) -> FpsCamera {
        FpsCamera {
            sensitivity,
            rotate_parent_y: false,
        }
    }

    pub fn new_rotate_parent_y(sensitivity: f32) -> FpsCamera {
        FpsCamera {
            sensitivity,
            rotate_parent_y: true,
        }
    }
}

fn setup(
    query: Query<(&Camera, Option<&ShowCursor>), With<FpsCamera>>,
    mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let Ok((camera, show_cursor)) = query.single() else {
        return;
    };
    if camera.is_active && show_cursor.is_none() {
        primary_cursor_options.grab_mode = CursorGrabMode::Locked;
        primary_cursor_options.visible = false;
    } else {
        primary_cursor_options.grab_mode = CursorGrabMode::None;
        primary_cursor_options.visible = true;
    }
}

fn handle_fps_cameras(
    mut query: Query<(Entity, &FpsCamera, Option<&ChildOf>)>,
    mut motion_evr: MessageReader<MouseMotion>,
    mut transform_query: Query<&mut Transform>,
    time: Res<Time>,
) {
    for (entity, free_cam, maybe_parent) in &mut query {
        for ev in motion_evr.read() {
            let rotation_dir = -ev.delta * free_cam.sensitivity * time.delta_secs();
            if let Some(parent) = maybe_parent
                && free_cam.rotate_parent_y
            {
                let Ok([mut transform, mut parent_transform]) =
                    transform_query.get_many_mut([entity, parent.parent()])
                else {
                    continue;
                };
                transform.rotate_axis(Dir3::X, rotation_dir.y);
                parent_transform.rotate_axis(Dir3::Y, rotation_dir.x);
            } else {
                let Ok(mut transform) = transform_query.get_mut(entity) else {
                    continue;
                };
                let right = transform.right();
                transform.rotate_axis(right, rotation_dir.y);
                transform.rotate_axis(Dir3::Y, rotation_dir.x);
            }
        }
    }
}
