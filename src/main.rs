use std::env;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;

use crate::camera::CameraPlugin;
use crate::day_night::DayNightPlugin;
use crate::debug::DebugPlugin;
use crate::levels::LevelPlugin;
use crate::player::PlayerPlugin;

mod camera;
mod day_night;
mod debug;
mod levels;
mod math;
mod player;
mod z_indecies;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

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
        .add_plugins((
            PlayerPlugin,
            LevelPlugin,
            CameraPlugin,
            DebugPlugin,
            DayNightPlugin,
        ))
        .run();
}
