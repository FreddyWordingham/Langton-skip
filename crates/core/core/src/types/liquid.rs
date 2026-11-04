use bevy::{prelude::*, sprite_render::TileData};

use crate::traits::{MenuItem, Tile};

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

impl MenuItem for Liquid {
    const ALL: &'static [Self] = &[Self::Water];

    fn label(&self) -> &'static str {
        match self {
            Self::Water => "Water",
        }
    }

    fn menu_sprite_path(&self) -> &'static str {
        match self {
            Self::Water => "sprites/tiles/liquids/water.png",
        }
    }

    fn menu_sprite_color(&self) -> Color {
        match self {
            Self::Water => Color::WHITE,
        }
    }
}
