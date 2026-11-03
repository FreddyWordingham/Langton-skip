use arc_core::{Construct, Feature, Item, Liquid, Terrain};
use bevy::prelude::*;

#[derive(Resource)]
pub enum ActiveSpawnSelection {
    Terrain(Terrain),
    Liquid(Liquid),
    Feature(Feature),
    Construct(Construct),
    Item(Item),
}
