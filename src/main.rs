use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::prelude::RapierPhysicsPlugin;

use crate::levels::LevelPlugin;
use crate::player::PlayerPlugin;

mod levels;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }),
            TilemapPlugin,
            EditorPlugin::default(),
            RapierPhysicsPlugin::default(),
        ))
        .add_plugins((PlayerPlugin, LevelPlugin))
        .run();
}
