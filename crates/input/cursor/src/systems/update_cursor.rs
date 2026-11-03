use arc_camera::MainCamera;
use arc_draw::TILE_SIZE;
use arc_map::Map;
use bevy::{
    picking::{hover::HoverMap, pointer::PointerId},
    prelude::*,
    window::PrimaryWindow,
};

use crate::resources::Cursor;

pub(crate) fn update_cursor(
    mut cursor: ResMut<Cursor>,
    hover_map: Res<HoverMap>,
    map: Res<Map>,
    ui_nodes: Query<(), With<Node>>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    main_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = *main_camera;

    let over_ui = hover_map
        .get(&PointerId::Mouse)
        .is_some_and(|hits| hits.keys().any(|&ent| ui_nodes.contains(ent)));
    if over_ui {
        if cursor.curr() != None || cursor.prev() != None {
            cursor.set_curr(None);
        }
        return;
    }

    let coord = primary_window
        .cursor_position()
        .and_then(|viewport_position| camera.viewport_to_world_2d(camera_transform, viewport_position).ok())
        .and_then(|world_position| {
            if world_position.x < 0.0 || world_position.y < 0.0 {
                return None;
            }
            let coord = (world_position / TILE_SIZE).floor().as_uvec2();

            let map_size = map.size();
            if coord.x >= map_size.x || coord.y >= map_size.y {
                return None;
            }
            Some(coord)
        });

    if cursor.curr() != coord || cursor.prev() != coord {
        cursor.set_curr(coord);
    }
}
