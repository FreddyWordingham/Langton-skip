use arc_content::Terrain;
use bevy::prelude::*;

#[derive(Message)]
pub struct SetTerrain {
    pub coord: UVec2,
    pub terrain: Option<Terrain>,
}
