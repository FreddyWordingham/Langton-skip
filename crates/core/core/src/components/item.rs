use bevy::prelude::*;

use crate::{traits::MenuItem, types::Count};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Iron,
}

impl Item {
    fn sprite_path(&self, count: Count) -> &'static str {
        match count {
            Count::Single => "sprites/buildings/drill/single.png",
            Count::Few => "sprites/buildings/drill/few.png",
            Count::Many => "sprites/buildings/drill/many.png",
        }
    }
}

impl MenuItem for Item {
    const ALL: &'static [Self] = &[Self::Iron];

    fn label(&self) -> &'static str {
        match self {
            Self::Iron => "Iron",
        }
    }

    fn menu_sprite_path(&self) -> &'static str {
        match self {
            Self::Iron => "sprites/items/iron/few.png",
        }
    }
}
