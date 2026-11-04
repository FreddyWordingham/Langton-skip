use arc_core::{Floor, Liquid, TILE_SIZE_VEC2, Terrain};
use arc_geometry::Size;
use arc_grid::{DirtyTrackingGrid, PlacementGrid};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    size: Size,
    terrain: DirtyTrackingGrid<Option<Terrain>>,
    floor: DirtyTrackingGrid<Option<Floor>>,
    liquid: DirtyTrackingGrid<Option<Liquid>>,
    constructs: PlacementGrid,
}

impl Map {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            terrain: DirtyTrackingGrid::new(size, Some(Terrain::Soil)),
            floor: DirtyTrackingGrid::new(size, None),
            liquid: DirtyTrackingGrid::new(size, None),
            constructs: PlacementGrid::new(size),
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn center(&self) -> Vec2 {
        self.size.as_vec2() * 0.5 * TILE_SIZE_VEC2
    }

    pub fn terrain(&self) -> &DirtyTrackingGrid<Option<Terrain>> {
        &self.terrain
    }

    pub fn floor(&self) -> &DirtyTrackingGrid<Option<Floor>> {
        &self.floor
    }

    pub fn liquid(&self) -> &DirtyTrackingGrid<Option<Liquid>> {
        &self.liquid
    }

    pub fn constructs(&self) -> &PlacementGrid {
        &self.constructs
    }

    pub(crate) fn terrain_mut(&mut self) -> &mut DirtyTrackingGrid<Option<Terrain>> {
        &mut self.terrain
    }

    pub(crate) fn floor_mut(&mut self) -> &mut DirtyTrackingGrid<Option<Floor>> {
        &mut self.floor
    }

    pub(crate) fn liquid_mut(&mut self) -> &mut DirtyTrackingGrid<Option<Liquid>> {
        &mut self.liquid
    }

    pub(crate) fn constructs_mut(&mut self) -> &mut PlacementGrid {
        &mut self.constructs
    }

    pub fn terrain_is_dirty(&self) -> bool {
        self.terrain.is_dirty()
    }

    pub fn floor_is_dirty(&self) -> bool {
        self.floor.is_dirty()
    }

    pub fn liquid_is_dirty(&self) -> bool {
        self.liquid.is_dirty()
    }

    pub fn terrain_drain_dirty(&mut self) -> impl Iterator<Item = UVec2> + '_ {
        self.terrain.drain_dirty()
    }

    pub fn floor_drain_dirty(&mut self) -> impl Iterator<Item = UVec2> + '_ {
        self.floor.drain_dirty()
    }

    pub fn liquid_drain_dirty(&mut self) -> impl Iterator<Item = UVec2> + '_ {
        self.liquid.drain_dirty()
    }
}
