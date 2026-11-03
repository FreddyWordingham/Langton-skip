#![allow(clippy::needless_pass_by_value)]

mod components;
mod types;

pub use crate::{
    components::{Construct, Feature, Item},
    types::{Liquid, Terrain},
};
