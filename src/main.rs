use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;

use crate::camera::CameraPlugin;
use crate::levels::LevelPlugin;
use crate::player::PlayerPlugin;

mod camera;
mod debug;
mod levels;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            TilemapPlugin,
            // EditorPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((PlayerPlugin, LevelPlugin, CameraPlugin))
        .run();
}
