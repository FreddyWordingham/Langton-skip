use arc_core::{Construct, Direction};
use bevy::prelude::*;

#[derive(Message)]
pub struct SpawnConstruct {
    pub coord: UVec2,
    pub construct: Construct,
    pub direction: Direction,
}
