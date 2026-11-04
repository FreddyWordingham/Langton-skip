use arc_geometry::{Aabb, Size};
use bevy::prelude::*;

use crate::{components::Direction, traits::MenuItem};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Construct {
    Drill,
}

impl Construct {
    fn sprite_path(&self, direction: Direction) -> &'static str {
        match direction {
            Direction::North => "sprites/buildings/drill/north.png",
            Direction::East => "sprites/buildings/drill/east.png",
            Direction::South => "sprites/buildings/drill/south.png",
            Direction::West => "sprites/buildings/drill/west.png",
        }
    }

    pub fn footprint(&self, coord: UVec2, direction: Direction) -> Aabb {
        let size = if direction == Direction::North || direction == Direction::South {
            self.size()
        } else {
            let s = self.size();
            Size::new(s.height(), s.width())
        };
        Aabb::new(coord, coord + size.as_uvec2() - UVec2::ONE)
    }

    fn size(&self) -> Size {
        match self {
            Self::Drill => Size::new(3, 3),
        }
    }
}

impl MenuItem for Construct {
    const ALL: &[Self] = &[Self::Drill];

    fn label(&self) -> &'static str {
        match self {
            Self::Drill => "Drill",
        }
    }

    fn menu_sprite_path(&self) -> &'static str {
        match self {
            Self::Drill => "sprites/buildings/drill/north.png",
        }
    }

    fn menu_sprite_color(&self) -> Color {
        match self {
            Self::Drill => Color::WHITE,
        }
    }
}
