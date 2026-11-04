use bevy::{platform::collections::HashMap, prelude::*};

use crate::types::{Grid, Size};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementGrid {
    entities: Grid<Option<Entity>>,
    placements: HashMap<Entity, URect>,
}

impl PlacementGrid {
    pub fn new(size: Size) -> Self {
        Self {
            entities: Grid::new(size, None),
            placements: HashMap::default(),
        }
    }

    pub fn size(&self) -> Size {
        self.entities.size()
    }
}
