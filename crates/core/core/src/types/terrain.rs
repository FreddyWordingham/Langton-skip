use bevy::{prelude::*, sprite_render::TileData};

use crate::traits::{MenuItem, Tile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terrain {
    Sand,
    Soil,
}

impl Tile for Terrain {
    fn tile_data(&self, coord: UVec2) -> TileData {
        let atlas_x = coord.x % 32;
        let atlas_y = (32 - (coord.y % 32)) % 32;
        let index = (atlas_x + atlas_y * 32) as u16;

        match self {
            Self::Sand => TileData {
                tileset_index: index,
                ..default()
            },
            Self::Soil => TileData {
                tileset_index: 1024 + index,
                ..default()
            },
        }
    }
}

impl MenuItem for Terrain {
    const ALL: &'static [Self] = &[Self::Soil, Self::Sand];

    fn label(&self) -> &'static str {
        match self {
            Self::Soil => "Soil",
            Self::Sand => "Sand",
        }
    }

    fn menu_sprite_path(&self) -> &'static str {
        match self {
            Self::Soil => "sprites/tiles/terrain/soil.png",
            Self::Sand => "sprites/tiles/terrain/sand.png",
        }
    }
}
