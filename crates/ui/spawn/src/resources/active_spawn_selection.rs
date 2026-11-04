use arc_core::{Construct, Feature, Floor, Item, Liquid, Terrain};
use bevy::prelude::*;

#[derive(Resource)]
pub enum ActiveSpawnSelection {
    Terrain(Terrain),
    Floor(Floor),
    Liquid(Liquid),
    Feature(Feature),
    Construct(Construct),
    Item(Item),
}
