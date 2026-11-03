use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpawnUpdate {
    Spawn,
    Sync,
    Trigger,
    Despawn,
}
