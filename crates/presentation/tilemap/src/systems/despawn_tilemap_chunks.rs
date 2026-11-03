use bevy::prelude::*;

use crate::{components::TilemapChunkMarker, resources::TilemapChunks, traits::TilemapLayer};

pub(crate) fn despawn_tilemap_chunks<L: TilemapLayer>(
    mut commands: Commands,
    tilemap_q: Query<Entity, With<TilemapChunkMarker<L>>>,
) {
    for ent in &tilemap_q {
        commands.entity(ent).despawn();
    }
    commands.remove_resource::<TilemapChunks<L>>();
}
