#![allow(clippy::needless_pass_by_value)]

mod constants;
mod traits;

pub use crate::{
    constants::{TILE_SIZE, TILE_SIZE_UVEC2, TILE_SIZE_VEC2},
    traits::{MenuItem, Tile},
};
