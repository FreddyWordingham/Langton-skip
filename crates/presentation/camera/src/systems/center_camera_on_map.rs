use arc_map::Map;
use bevy::prelude::*;

use crate::components::MainCamera;

pub(crate) fn center_camera_on_map(map: Res<Map>, mut main_camera_q: Query<&mut Transform, With<MainCamera>>) {
    for mut transform in &mut main_camera_q {
        transform.translation = map.center().extend(transform.translation.z);
    }
}
