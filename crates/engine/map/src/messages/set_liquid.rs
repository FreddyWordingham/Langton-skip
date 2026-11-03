use arc_core::Liquid;
use bevy::prelude::*;

#[derive(Message)]
pub struct SetLiquid {
    pub coord: UVec2,
    pub liquid: Option<Liquid>,
}
