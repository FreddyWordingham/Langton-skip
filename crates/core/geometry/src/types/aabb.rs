use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Aabb {
    min: UVec2,
    max: UVec2,
}

impl Aabb {
    pub const fn new(min: UVec2, max: UVec2) -> Self {
        assert!(min.x <= max.x && min.y <= max.y, "Aabb min must be less than or equal to max");
        Self { min, max }
    }

    pub fn min(&self) -> UVec2 {
        self.min
    }

    pub fn max(&self) -> UVec2 {
        self.max
    }

    pub fn in_bounds(&self, coord: UVec2) -> bool {
        coord.x >= self.min.x && coord.x <= self.max.x && coord.y >= self.min.y && coord.y <= self.max.y
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = UVec2> {
        (self.min.y..=self.max.y).flat_map(move |y| (self.min.x..=self.max.x).map(move |x| uvec2(x, y)))
    }
}
