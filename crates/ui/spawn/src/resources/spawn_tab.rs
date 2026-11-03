use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpawnTab {
    Terrain,
    Liquid,
    Feature,
    Construct,
    Item,
}

impl SpawnTab {
    pub(crate) const ALL: [Self; 5] = [Self::Terrain, Self::Liquid, Self::Feature, Self::Construct, Self::Item];

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Terrain => "Terrain",
            Self::Liquid => "Liquid",
            Self::Feature => "Feature",
            Self::Construct => "Construct",
            Self::Item => "Item",
        }
    }
}
