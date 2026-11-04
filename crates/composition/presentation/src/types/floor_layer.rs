use arc_core::{TILE_SIZE_UVEC2, Tile};
use arc_grid::Size;
use arc_map::Map;
use arc_tilemap::{TilemapConfig, TilemapLayer};
use bevy::{
    image::{ImageArrayLayout, ImageSampler},
    prelude::*,
    sprite_render::{AlphaMode2d, TileData},
};

pub(crate) struct FloorLayer;

impl TilemapLayer for FloorLayer {
    type Source = Map;

    fn size(source: &Self::Source) -> Size {
        source.floor().size()
    }

    fn tile_data(source: &Self::Source, coord: UVec2) -> Option<TileData> {
        source.floor()[coord].map(|floor| floor.tile_data(coord))
    }

    fn is_dirty(source: &Self::Source) -> bool {
        source.floor_is_dirty()
    }

    fn drain_dirty(source: &mut Self::Source) -> Vec<UVec2> {
        source.floor_drain_dirty().collect()
    }
}

impl FloorLayer {
    pub(crate) fn config() -> TilemapConfig {
        TilemapConfig {
            chunk_size: UVec2::splat(32),
            tile_display_size: TILE_SIZE_UVEC2,
            tileset_path: "atlases/floor.png".to_owned(),
            array_layout: Some(ImageArrayLayout::GridSize {
                tile_height_pixels: 128,
                tile_width_pixels: 128,
            }),
            z_index: -2.0,
            sampler: ImageSampler::nearest(),
            alpha_mode: AlphaMode2d::Mask(0.5),
        }
    }
}
