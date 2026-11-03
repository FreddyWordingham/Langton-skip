use arc_content::{Construct, Feature, Item, Terrain};
use bevy::prelude::*;

#[derive(Resource)]
pub enum ActiveSpawnSelection {
    Terrain(Terrain),
    Feature(Feature),
    Construct(Construct),
    Item(Item),
}
