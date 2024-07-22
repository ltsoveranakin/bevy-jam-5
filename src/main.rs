use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;

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
            // EditorPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((PlayerPlugin, LevelPlugin))
        .add_systems(Startup, init_camera)
        .run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
