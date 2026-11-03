use bevy::prelude::*;

use crate::{messages::SetLiquid, resources::Map};

pub(crate) fn handle_set_liquid(mut set_liquid_msg: MessageReader<SetLiquid>, mut map: ResMut<Map>) {
    for msg in set_liquid_msg.read() {
        map.liquid_mut()[msg.coord] = msg.liquid;
    }
}
