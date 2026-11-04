use arc_geometry::Aabb;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size(UVec2);

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self(uvec2(width, height))
    }

    pub fn width(&self) -> u32 {
        self.0.x
    }

    pub fn height(&self) -> u32 {
        self.0.y
    }

    pub fn len(&self) -> usize {
        self.0.x as usize * self.0.y as usize
    }

    pub fn is_empty(&self) -> bool {
        self.0.x == 0 || self.0.y == 0
    }

    pub fn aabb_in_bounds(&self, aabb: Aabb) -> bool {
        self.in_bounds(aabb.min()) && self.in_bounds(aabb.max())
    }

    pub fn in_bounds(&self, coord: UVec2) -> bool {
        coord.x < self.0.x && coord.y < self.0.y
    }

    pub fn try_in_bounds(&self, point: IVec2) -> Option<UVec2> {
        UVec2::try_from(point).ok().filter(|&coord| self.in_bounds(coord))
    }

    pub fn index_to_coord(&self, index: usize) -> UVec2 {
        assert!(index < self.len(), "Index out of bounds");
        let x = (index % self.0.x as usize) as u32;
        let y = (index / self.0.x as usize) as u32;
        uvec2(x, y)
    }

    pub fn coord_to_index(&self, coord: UVec2) -> usize {
        assert!(self.in_bounds(coord), "Coordinate out of bounds");
        (coord.y as usize * self.0.x as usize) + coord.x as usize
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = UVec2> {
        (0..self.0.y).flat_map(move |y| (0..self.0.x).map(move |x| UVec2::new(x, y)))
    }

    pub fn as_uvec2(&self) -> UVec2 {
        self.0
    }

    pub fn as_vec2(&self) -> Vec2 {
        self.0.as_vec2()
    }
}

impl From<UVec2> for Size {
    fn from(value: UVec2) -> Self {
        Self(value)
    }
}
