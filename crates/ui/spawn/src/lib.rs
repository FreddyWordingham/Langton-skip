#![allow(clippy::needless_pass_by_value)]

mod components;
mod plugin;
mod resources;
mod sets;
mod systems;
mod utils;

pub use crate::{
    plugin::SpawnPlugin,
    resources::{ActiveSpawnSelection, SpawnTab},
    sets::SpawnUpdate,
};
