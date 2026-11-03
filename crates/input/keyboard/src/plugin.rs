use bevy::{input::InputSystems, prelude::*};

use crate::{sets::KeyboardPreUpdate, systems::control_camera};

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PreUpdate, KeyboardPreUpdate::Input.after(InputSystems))
            .add_systems(PreUpdate, control_camera.in_set(KeyboardPreUpdate::Input));
    }
}
