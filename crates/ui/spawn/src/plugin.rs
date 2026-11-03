use bevy::prelude::*;

use crate::{
    resources::{ActiveSpawnSelection, SpawnTab},
    sets::SpawnUpdate,
    systems::{despawn_ui, spawn_ui, trigger_despawn, trigger_spawn, update_spawn_selection_buttons},
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                SpawnUpdate::Spawn,
                SpawnUpdate::Sync,
                SpawnUpdate::Trigger,
                SpawnUpdate::Despawn,
            )
                .chain(),
        )
        .insert_resource(SpawnTab::Terrain)
        .add_systems(
            Update,
            (
                spawn_ui.run_if(resource_added::<SpawnTab>).in_set(SpawnUpdate::Spawn),
                update_spawn_selection_buttons
                    .run_if(resource_exists_and_changed::<SpawnTab>)
                    .in_set(SpawnUpdate::Sync),
                (
                    trigger_spawn.run_if(resource_exists::<ActiveSpawnSelection>),
                    trigger_despawn.run_if(resource_exists::<SpawnTab>),
                )
                    .in_set(SpawnUpdate::Trigger),
                despawn_ui.run_if(resource_removed::<SpawnTab>).in_set(SpawnUpdate::Despawn),
            ),
        );
    }
}
