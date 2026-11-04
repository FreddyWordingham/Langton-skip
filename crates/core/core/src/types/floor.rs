use bevy::{prelude::*, sprite_render::TileData};

use crate::traits::{MenuItem, Tile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Floor {
    Flagstone,
    Wood,
    Straw,
}

impl Tile for Floor {
    fn tile_data(&self, coord: UVec2) -> TileData {
        let atlas_x = coord.x % 8;
        let atlas_y = (8 - (coord.y % 8)) % 8;
        let index = (atlas_x + atlas_y * 8) as u16;

        match self {
            Self::Flagstone => TileData {
                tileset_index: index,
                ..default()
            },
            Self::Wood => TileData {
                tileset_index: 64 + index,
                color: Color::srgb(0.8, 0.6, 0.4),
                ..default()
            },
            Self::Straw => TileData {
                tileset_index: 128 + index,
                ..default()
            },
        }
    }
}

impl MenuItem for Floor {
    const ALL: &'static [Self] = &[Self::Flagstone, Self::Wood, Self::Straw];

    fn label(&self) -> &'static str {
        match self {
            Self::Flagstone => "Flagstone",
            Self::Wood => "Wood",
            Self::Straw => "Straw",
        }
    }

    fn menu_sprite_path(&self) -> &'static str {
        match self {
            Self::Flagstone => "sprites/tiles/floors/flagstone.png",
            Self::Wood => "sprites/tiles/floors/wood.png",
            Self::Straw => "sprites/tiles/floors/straw.png",
        }
    }

    fn menu_sprite_color(&self) -> Color {
        match self {
            Self::Flagstone => Color::WHITE,
            Self::Wood => Color::srgb(0.8, 0.6, 0.4),
            Self::Straw => Color::WHITE,
        }
    }
}
