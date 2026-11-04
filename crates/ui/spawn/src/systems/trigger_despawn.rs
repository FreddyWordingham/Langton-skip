use arc_cursor::Cursor;
use arc_map::{SetFloor, SetLiquid, SetTerrain};
use bevy::prelude::*;

use crate::resources::SpawnTab;

pub(crate) fn trigger_despawn(
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    spawn_tab: Res<SpawnTab>,
    mut set_terrain_msg: MessageWriter<SetTerrain>,
    mut set_floor_msg: MessageWriter<SetFloor>,
    mut set_liquid_msg: MessageWriter<SetLiquid>,
) {
    let Some(coord) = cursor.curr() else { return };

    if mouse.just_pressed(MouseButton::Right) {
        match *spawn_tab {
            SpawnTab::Terrain => {
                set_terrain_msg.write(SetTerrain { coord, terrain: None });
            }
            SpawnTab::Floor => {
                set_floor_msg.write(SetFloor { coord, floor: None });
            }
            SpawnTab::Liquid => {
                set_liquid_msg.write(SetLiquid { coord, liquid: None });
            }
            _ => {
                unimplemented!()
            }
        }
    };
}
