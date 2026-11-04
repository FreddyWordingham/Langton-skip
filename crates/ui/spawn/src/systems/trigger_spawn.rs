use arc_cursor::Cursor;
use arc_map::{SetFloor, SetLiquid, SetTerrain};
use bevy::prelude::*;

use crate::resources::ActiveSpawnSelection;

pub(crate) fn trigger_spawn(
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    active_spawn_selection: Res<ActiveSpawnSelection>,
    mut set_terrain_msg: MessageWriter<SetTerrain>,
    mut set_floor_msg: MessageWriter<SetFloor>,
    mut set_liquid_msg: MessageWriter<SetLiquid>,
) {
    let Some(coord) = cursor.curr() else { return };

    if mouse.just_pressed(MouseButton::Left) {
        match *active_spawn_selection {
            ActiveSpawnSelection::Terrain(terrain) => {
                set_terrain_msg.write(SetTerrain {
                    coord,
                    terrain: Some(terrain),
                });
            }
            ActiveSpawnSelection::Floor(floor) => {
                set_floor_msg.write(SetFloor {
                    coord,
                    floor: Some(floor),
                });
            }
            ActiveSpawnSelection::Liquid(liquid) => {
                set_liquid_msg.write(SetLiquid {
                    coord,
                    liquid: Some(liquid),
                });
            }
            _ => {
                unimplemented!()
            }
        }
    };
}
