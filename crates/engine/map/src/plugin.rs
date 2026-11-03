use bevy::prelude::*;

use crate::{
    messages::{CreateMap, DestroyMap, SetLiquid, SetTerrain},
    resources::Map,
    sets::MapFixedUpdate,
    systems::{handle_create_map, handle_destroy_map, handle_set_liquid, handle_set_terrain},
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                MapFixedUpdate::Create,
                MapFixedUpdate::MutateTerrain,
                MapFixedUpdate::MutateLiquid,
                MapFixedUpdate::Destroy,
            )
                .chain(),
        )
        .add_message::<CreateMap>()
        .add_message::<SetTerrain>()
        .add_message::<SetLiquid>()
        .add_message::<DestroyMap>()
        .add_systems(
            FixedUpdate,
            (
                handle_create_map.in_set(MapFixedUpdate::Create),
                handle_set_terrain
                    .run_if(resource_exists::<Map>)
                    .in_set(MapFixedUpdate::MutateTerrain),
                handle_set_liquid
                    .run_if(resource_exists::<Map>)
                    .in_set(MapFixedUpdate::MutateLiquid),
                handle_destroy_map.in_set(MapFixedUpdate::Destroy),
            ),
        );
    }
}
