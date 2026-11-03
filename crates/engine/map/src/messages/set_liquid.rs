use arc_content::Liquid;
use bevy::prelude::*;

#[derive(Message)]
pub struct SetLiquid {
    pub coord: UVec2,
    pub liquid: Option<Liquid>,
}
