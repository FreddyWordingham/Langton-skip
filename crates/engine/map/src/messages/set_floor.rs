use arc_core::Floor;
use bevy::prelude::*;

#[derive(Message)]
pub struct SetFloor {
    pub coord: UVec2,
    pub floor: Option<Floor>,
}
