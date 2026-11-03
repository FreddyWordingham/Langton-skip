use bevy::prelude::*;

use crate::resources::Cursor;

pub(crate) fn reset_cursor(mut cursor: ResMut<Cursor>) {
    cursor.reset();
}
