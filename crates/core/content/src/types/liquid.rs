use arc_draw::{Draw, Tile};
use bevy::{prelude::*, sprite_render::TileData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Liquid {
    Water,
}

impl Tile for Liquid {
    fn tile_data(&self, _coord: UVec2) -> TileData {
        match self {
            Self::Water => TileData {
                tileset_index: 0,
                ..default()
            },
        }
    }
}

impl Draw for Liquid {
    const ALL: &'static [Self] = &[Self::Water];

    fn name(&self) -> &'static str {
        match self {
            Self::Water => "Water",
        }
    }

    fn menu_sprite_path(&self) -> &'static str {
        match self {
            Self::Water => "sprites/tiles/liquids/water.png",
        }
    }
}
