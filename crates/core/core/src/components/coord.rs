use bevy::prelude::*;

use crate::constants::TILE_SIZE_VEC2;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(UVec2);

impl Coord {
    pub fn new(coord: UVec2) -> Self {
        Coord(coord)
    }

    pub fn as_uvec2(self) -> UVec2 {
        self.0
    }

    pub fn bottom_left(self) -> Vec2 {
        self.0.as_vec2() * TILE_SIZE_VEC2
    }

    pub fn center(self) -> Vec2 {
        (self.0.as_vec2() + Vec2::splat(0.5)) * TILE_SIZE_VEC2
    }
}
