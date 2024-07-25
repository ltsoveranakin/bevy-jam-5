use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;

use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::levels::LevelPlugin;
use crate::player::PlayerPlugin;

mod camera;
mod debug;
mod levels;
mod math;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(800., 600.),
                        ..default()
                    }),
                    ..default()
                }),
            TilemapPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default().disabled(),
        ))
        .add_plugins((PlayerPlugin, LevelPlugin, CameraPlugin, DebugPlugin))
        .run();
}
