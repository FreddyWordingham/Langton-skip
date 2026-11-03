use bevy::prelude::*;

use crate::{components::Item, traits::MenuItem};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    Ore(Item),
}

impl MenuItem for Feature {
    const ALL: &'static [Self] = &[Self::Ore(Item::Iron)];

    fn label(&self) -> &'static str {
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
