use arc_draw::TILE_SIZE_VEC2;
use arc_grid::DirtyTrackingGrid;
use arc_terrain::Terrain;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    terrain: DirtyTrackingGrid<Option<Terrain>>,
}

impl Map {
    pub fn new(size: UVec2) -> Self {
        Self {
            terrain: DirtyTrackingGrid::new(size, Some(Terrain::Soil)),
        }
    }

    pub fn size(&self) -> UVec2 {
        self.terrain.size()
    }

    pub fn center(&self) -> Vec2 {
        self.terrain.size().as_vec2() * 0.5 * TILE_SIZE_VEC2
    }

    pub fn terrain(&self) -> &DirtyTrackingGrid<Option<Terrain>> {
        &self.terrain
    }

    pub fn terrain_mut(&mut self) -> &mut DirtyTrackingGrid<Option<Terrain>> {
        &mut self.terrain
    }
}
