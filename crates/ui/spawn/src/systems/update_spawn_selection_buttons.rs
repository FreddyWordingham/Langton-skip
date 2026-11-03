use bevy::prelude::*;

use crate::{components::SpawnSelectionPanel, resources::SpawnTab, utils::spawn_selection_buttons};

pub(crate) fn update_spawn_selection_buttons(
    mut commands: Commands,
    assets: Res<AssetServer>,
    spawn_tab: Res<SpawnTab>,
    spawn_selection_panel_q: Query<Entity, With<SpawnSelectionPanel>>,
) {
    for ent in &spawn_selection_panel_q {
        commands.entity(ent).despawn_children().with_children(|parent| {
            spawn_selection_buttons(parent, &assets, *spawn_tab);
        });
    }
}
