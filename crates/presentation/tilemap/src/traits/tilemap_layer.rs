use arc_grid::Size;
use bevy::{ecs::component::Mutable, prelude::*, sprite_render::TileData};

pub trait TilemapLayer: 'static {
    type Source: Resource<Mutability = Mutable>;

    fn size(source: &Self::Source) -> Size;

    fn tile_data(source: &Self::Source, coord: UVec2) -> Option<TileData>;

    fn is_dirty(source: &Self::Source) -> bool;

    fn drain_dirty(source: &mut Self::Source) -> Vec<UVec2>;
}
