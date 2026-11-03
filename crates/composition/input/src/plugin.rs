use arc_cursor::CursorPlugin;
use arc_keyboard::KeyboardPlugin;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CursorPlugin, KeyboardPlugin));
    }
}
