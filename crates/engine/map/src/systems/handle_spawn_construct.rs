use arc_core::Coord;
use bevy::prelude::*;

use crate::{messages::SpawnConstruct, resources::Map};

pub(crate) fn handle_spawn_construct(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut spawn_construct_msg: MessageReader<SpawnConstruct>,
) {
    for msg in spawn_construct_msg.read() {
        let footprint = msg.construct.footprint(msg.coord, msg.direction);
        if !map.constructs().can_place(footprint) {
            warn!("Cannot place construct at {:?}", msg.coord);
            continue;
        }

        let ent = commands.spawn((msg.construct, Coord::new(msg.coord), msg.direction)).id();
        map.constructs_mut().place(ent, footprint);
    }
}
