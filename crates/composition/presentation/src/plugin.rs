use arc_camera::CameraPlugin;
use arc_tilemap::TilemapPlugin;
use bevy::prelude::*;

use crate::types::{FloorLayer, LiquidLayer, TerrainLayer};

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CameraPlugin,
            TilemapPlugin::<FloorLayer>::new(FloorLayer::config()),
            TilemapPlugin::<LiquidLayer>::new(LiquidLayer::config()),
            TilemapPlugin::<TerrainLayer>::new(TerrainLayer::config()),
        ));
    }
}
