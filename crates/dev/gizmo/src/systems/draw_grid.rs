use arc_core::TILE_SIZE_VEC2;
use arc_map::Map;
use bevy::prelude::*;

pub(crate) fn draw_grid(mut gizmos: Gizmos, map: Res<Map>) {
    gizmos
        .grid_2d(
            Isometry2d::from_translation(map.center()),
            map.size().as_uvec2(),
            TILE_SIZE_VEC2,
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}
