use arc_content::{Liquid, Terrain};
use arc_draw::TILE_SIZE_VEC2;
use arc_grid::DirtyTrackingGrid;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    terrain: DirtyTrackingGrid<Option<Terrain>>,
    liquid: DirtyTrackingGrid<Option<Liquid>>,
}

impl Map {
    pub fn new(size: UVec2) -> Self {
        Self {
            terrain: DirtyTrackingGrid::new(size, Some(Terrain::Soil)),
            liquid: DirtyTrackingGrid::new(size, Some(Liquid::Water)),
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

    pub fn liquid(&self) -> &DirtyTrackingGrid<Option<Liquid>> {
        &self.liquid
    }

    pub fn terrain_mut(&mut self) -> &mut DirtyTrackingGrid<Option<Terrain>> {
        &mut self.terrain
    }

    pub fn liquid_mut(&mut self) -> &mut DirtyTrackingGrid<Option<Liquid>> {
        &mut self.liquid
    }
}
