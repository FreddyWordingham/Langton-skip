use arc_draw::Draw;
use bevy::prelude::*;

use crate::components::Item;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    Ore(Item),
}

impl Draw for Feature {
    const ALL: &'static [Self] = &[Self::Ore(Item::Iron)];

    fn name(&self) -> &'static str {
        match self {
            Self::Ore(item) => match item {
                Item::Iron => "Iron Ore",
            },
        }
    }

    fn menu_sprite_path(&self) -> &'static str {
        match self {
            Self::Ore(item) => item.menu_sprite_path(),
        }
    }
}
