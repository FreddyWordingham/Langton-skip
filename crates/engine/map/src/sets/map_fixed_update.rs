use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapFixedUpdate {
    Create,
    MutateTerrain,
    MutateFloors,
    MutateLiquids,
    MutateConstructs,
    Destroy,
}
