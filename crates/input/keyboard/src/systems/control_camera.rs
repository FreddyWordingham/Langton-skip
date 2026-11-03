use arc_camera::CameraControl;
use bevy::prelude::*;

use crate::constants::{PAN_EAST_KEY, PAN_NORTH_KEY, PAN_SOUTH_KEY, PAN_WEST_KEY, ZOOM_IN_KEY, ZOOM_OUT_KEY};

pub(crate) fn control_camera(keyboard: Res<ButtonInput<KeyCode>>, mut camera_control: ResMut<CameraControl>) {
    camera_control.pan = Vec2::new(
        axis(keyboard.pressed(PAN_EAST_KEY), keyboard.pressed(PAN_WEST_KEY)),
        axis(keyboard.pressed(PAN_NORTH_KEY), keyboard.pressed(PAN_SOUTH_KEY)),
    );

    camera_control.zoom = axis(keyboard.pressed(ZOOM_IN_KEY), keyboard.pressed(ZOOM_OUT_KEY));
}

fn axis(positive: bool, negative: bool) -> f32 {
    f32::from(u8::from(positive)) - f32::from(u8::from(negative))
}
