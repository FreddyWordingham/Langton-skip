use arc_draw::{Draw, Tile};
use bevy::{prelude::*, sprite_render::TileData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Liquid {
    Water,
}

impl Liquid {
    pub const ALL: [Self; 1] = [Self::Water];
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
    fn name(&self) -> &'static str {
        match self {
            Self::Water => "Water",
        }
    }

    fn sprite_path(&self) -> &'static str {
        match self {
            Self::Water => "sprites/tiles/terrain/misc.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Water => Color::srgb(0.3, 0.7, 0.9),
        }
    }
}
