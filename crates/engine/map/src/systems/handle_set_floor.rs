use bevy::prelude::*;

use crate::{messages::SetFloor, resources::Map};

pub(crate) fn handle_set_floor(mut set_floor_msg: MessageReader<SetFloor>, mut map: ResMut<Map>) {
    for msg in set_floor_msg.read() {
        map.floor_mut()[msg.coord] = msg.floor;
    }
}
