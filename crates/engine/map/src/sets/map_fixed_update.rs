use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapFixedUpdate {
    Create,
    MutateTerrain,
    MutateFloor,
    MutateLiquid,
    Destroy,
}
