use std::ops::{Index, IndexMut};

use bevy::{platform::collections::HashSet, prelude::*};

use crate::types::{Grid, Size};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirtyTrackingGrid<T> {
    grid: Grid<T>,
    dirty: HashSet<UVec2>,
}

impl<T> DirtyTrackingGrid<T> {
    pub fn new(size: Size, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            grid: Grid::new(size, value),
            dirty: HashSet::new(),
        }
    }

    pub fn size(&self) -> Size {
        self.grid.size()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    pub fn iter_with_coords(&self) -> impl Iterator<Item = (UVec2, &T)> {
        self.grid.iter_with_coords()
    }

    pub fn mark_dirty(&mut self, coord: UVec2) {
        if self.grid.size().in_bounds(coord) {
            self.dirty.insert(coord);
        }
    }

    pub fn is_dirty(&self) -> bool {
        !self.dirty.is_empty()
    }

    pub fn drain_dirty(&mut self) -> impl Iterator<Item = UVec2> + '_ {
        self.dirty.drain()
    }
}

impl<T> Index<UVec2> for DirtyTrackingGrid<T> {
    type Output = T;

    fn index(&self, coord: UVec2) -> &Self::Output {
        &self.grid[coord]
    }
}

impl<T> IndexMut<UVec2> for DirtyTrackingGrid<T> {
    fn index_mut(&mut self, coord: UVec2) -> &mut Self::Output {
        self.mark_dirty(coord);
        &mut self.grid[coord]
    }
}
