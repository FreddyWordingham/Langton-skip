use arc_grid::Size;
use bevy::{
    image::ImageLoaderSettings,
    platform::collections::HashMap,
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::{
    components::TilemapChunkMarker,
    resources::{TilemapChunks, TilemapSettings},
    traits::TilemapLayer,
};

pub(crate) fn spawn_tilemap_chunks<L: TilemapLayer>(
    mut commands: Commands,
    assets: Res<AssetServer>,
    tilemap_settings: Res<TilemapSettings<L>>,
    source: Res<L::Source>,
    tilemap_q: Query<Entity, With<TilemapChunkMarker<L>>>,
) {
    for ent in &tilemap_q {
        commands.entity(ent).despawn();
    }
    commands.remove_resource::<TilemapChunks<L>>();

    let config = tilemap_settings.config();
    let array_layout = config.array_layout;
    let sampler = config.sampler.clone();
    let tileset = assets
        .load_builder()
        .with_settings(move |settings: &mut ImageLoaderSettings| {
            settings.array_layout = array_layout;
            settings.sampler = sampler.clone();
        })
        .load(config.tileset_path.clone());

    let grid_size = L::size(&source);
    let mut ents = HashMap::new();
    let num_chunks = UVec2::new(
        grid_size.width().div_ceil(config.chunk_size.x),
        grid_size.height().div_ceil(config.chunk_size.y),
    );
    for chunk_coord in Size::from(num_chunks).iter_coords() {
        let origin = chunk_coord * config.chunk_size;
        let chunk_size = config.chunk_size.min(grid_size.as_uvec2() - origin);
        let tile_data: Vec<Option<TileData>> = Size::from(chunk_size)
            .iter_coords()
            .map(|local_coord| L::tile_data(&source, origin + local_coord))
            .collect();

        let center = (origin.as_vec2() + chunk_size.as_vec2() * 0.5) * config.tile_display_size.as_vec2();
        let ent = commands
            .spawn((
                TilemapChunkMarker::<L>::new(),
                TilemapChunk {
                    chunk_size,
                    tile_display_size: config.tile_display_size,
                    tileset: tileset.clone(),
                    alpha_mode: config.alpha_mode,
                },
                TilemapChunkTileData(tile_data),
                Transform::from_translation(center.extend(config.z_index)),
            ))
            .id();
        ents.insert(chunk_coord, ent);
    }
    commands.insert_resource(TilemapChunks::<L>::new(ents));
}
