use arc_draw::{TILE_SIZE_UVEC2, Tile};
use arc_map::Map;
use arc_tilemap::{TilemapConfig, TilemapLayer};
use bevy::{
    image::{ImageArrayLayout, ImageSampler},
    prelude::*,
    sprite_render::{AlphaMode2d, TileData},
};

pub(crate) struct LiquidLayer;

impl TilemapLayer for LiquidLayer {
    type Source = Map;

    fn size(source: &Self::Source) -> UVec2 {
        source.liquid().size()
    }

    fn tile_data(source: &Self::Source, coord: UVec2) -> Option<TileData> {
        source.liquid()[coord].map(|liquid| liquid.tile_data(coord))
    }

    fn is_dirty(source: &Self::Source) -> bool {
        source.liquid_is_dirty()
    }

    fn drain_dirty(source: &mut Self::Source) -> Vec<UVec2> {
        source.liquid_drain_dirty().collect()
    }
}

impl LiquidLayer {
    pub(crate) fn config() -> TilemapConfig {
        TilemapConfig {
            chunk_size: UVec2::splat(32),
            tile_display_size: TILE_SIZE_UVEC2,
            tileset_path: "atlases/liquid.png".to_owned(),
            array_layout: Some(ImageArrayLayout::GridSize {
                tile_height_pixels: 32,
                tile_width_pixels: 32,
            }),
            z_index: -1.0,
            sampler: ImageSampler::nearest(),
            alpha_mode: AlphaMode2d::Blend,
        }
    }
}
