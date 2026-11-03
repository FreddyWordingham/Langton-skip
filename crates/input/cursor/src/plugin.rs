use arc_map::Map;
use bevy::{input::InputSystems, prelude::*};

use crate::{
    sets::CursorPreUpdate,
    systems::{reset_cursor, update_cursor},
};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PreUpdate, CursorPreUpdate::Input.after(InputSystems))
            .add_systems(
                PreUpdate,
                (
                    reset_cursor.run_if(resource_removed::<Map>),
                    update_cursor.run_if(resource_exists::<Map>),
                )
                    .in_set(CursorPreUpdate::Input),
            );
    }
}
