use arc_geometry::Aabb;
use bevy::{platform::collections::HashMap, prelude::*};

use crate::types::{Grid, Size};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementGrid {
    entities: Grid<Option<Entity>>,
    placements: HashMap<Entity, Aabb>,
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

    pub fn entity_at(&self, coord: UVec2) -> Option<Entity> {
        if !self.size().in_bounds(coord) {
            return None;
        }
        self.entities[coord]
    }

    pub fn footprint(&self, entity: Entity) -> Aabb {
        self.placements[&entity]
    }

    pub fn can_place(&self, aabb: Aabb) -> bool {
        self.validate_area(aabb, None);
        true
    }

    pub fn place(&mut self, entity: Entity, aabb: Aabb) {
        if self.placements.contains_key(&entity) {
            panic!("Entity already placed: {:?}", entity);
        }

        self.validate_area(aabb, None);

        for coord in aabb.iter_coords() {
            self.entities[coord] = Some(entity);
        }
        self.placements.insert(entity, aabb);
    }

    pub fn remove(&mut self, entity: Entity) -> Aabb {
        let aabb = self.placements.remove(&entity).expect("Entity not placed");
        for coord in aabb.iter_coords() {
            self.entities[coord] = None;
        }
        aabb
    }

    pub fn relocate(&mut self, entity: Entity, aabb: Aabb) {
        let previous = self.placements.get(&entity).expect("Entity not placed");

        self.validate_area(aabb, Some(entity));

        for coord in previous.iter_coords() {
            self.entities[coord] = None;
        }

        for coord in aabb.iter_coords() {
            self.entities[coord] = Some(entity);
        }

        self.placements.insert(entity, aabb);
    }

    fn validate_area(&self, aabb: Aabb, ignored_entity: Option<Entity>) {
        if !self.size().aabb_in_bounds(aabb) {
            panic!("Aabb out of bounds: {:?}", aabb);
        }

        for coord in aabb.iter_coords() {
            if let Some(occupant) = self.entities[coord] {
                if Some(occupant) != ignored_entity {
                    panic!("Coordinate occupied: {:?} by {:?}", coord, occupant);
                }
            }
        }
    }
}
