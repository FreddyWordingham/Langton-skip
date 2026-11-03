use arc_draw::TILE_SIZE_VEC2;
use arc_map::Map;
use bevy::prelude::*;

pub(crate) fn draw_grid(mut gizmos: Gizmos, map: Res<Map>) {
    let size = map.terrain().size();

    gizmos
        .grid_2d(
            Isometry2d::from_translation((size.as_vec2() * 0.5) * TILE_SIZE_VEC2),
            size,
            TILE_SIZE_VEC2,
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}
