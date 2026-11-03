use arc_map::Map;
use bevy::{input::InputSystems, picking::PickingSystems, prelude::*};

use crate::{
    resources::Cursor,
    sets::CursorPreUpdate,
    systems::{reset_cursor, update_cursor},
};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            CursorPreUpdate::Input.after(InputSystems).after(PickingSystems::Hover),
        )
        .init_resource::<Cursor>()
        .add_systems(
            PreUpdate,
            (
                reset_cursor.run_if(resource_removed::<Map>),
                update_cursor.run_if(resource_exists::<Map>),
            )
                .in_set(CursorPreUpdate::Input),
        )
        .add_systems(Update, report_cursor.run_if(resource_changed::<Cursor>));
    }
}

fn report_cursor(cursor: Res<Cursor>) {
    println!("Cursor: {:?} - {:?}", cursor.prev(), cursor.curr());
}
