use std::ops::{Index, IndexMut};

use bevy::prelude::*;

use crate::types::Size;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    size: Size,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(size: Size, value: T) -> Self
    where
        T: Clone,
    {
        let len = size.len();
        Self {
            size,
            data: vec![value; len],
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_with_coords(&self) -> impl Iterator<Item = (UVec2, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(move |(index, value)| (self.size.index_to_coord(index), value))
    }
}

impl<T> Index<UVec2> for Grid<T> {
    type Output = T;

    fn index(&self, coord: UVec2) -> &Self::Output {
        let index = self.size.coord_to_index(coord);
        &self.data[index]
    }
}

impl<T> IndexMut<UVec2> for Grid<T> {
    fn index_mut(&mut self, coord: UVec2) -> &mut Self::Output {
        let index = self.size.coord_to_index(coord);
        &mut self.data[index]
    }
}
