#![allow(clippy::needless_pass_by_value)]

mod plugin;
mod resources;
mod sets;
mod systems;

pub use crate::{plugin::CursorPlugin, resources::Cursor, sets::CursorPreUpdate};
