use bevy::prelude::*;

use crate::{messages::SetTerrain, resources::Map};

pub(crate) fn handle_set_terrain(mut set_terrain_msg: MessageReader<SetTerrain>, mut map: ResMut<Map>) {
    for msg in set_terrain_msg.read() {
        map.terrain_mut()[msg.coord] = msg.terrain;
    }
}
