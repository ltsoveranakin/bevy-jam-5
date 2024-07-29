#[cfg(not(debug_assertions))]
extern crate console_error_panic_hook;

use std::env;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;

use crate::camera::CameraPlugin;
use crate::day_night::DayNightPlugin;
use crate::debug::DebugPlugin;
use crate::instruction_screen::InstructionScreenPlugin;
use crate::levels::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::timer::TimerPlugin;
use crate::win::WinGamePlugin;

mod camera;
mod day_night;
mod debug;
mod instruction_screen;
mod levels;
mod math;
mod player;
mod timer;
mod win;
mod z_indices;

fn main() {
    #[cfg(not(debug_assertions))]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    #[cfg(debug_assertions)]
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
                        title: "Don't Chill Out".into(),
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
            TimerPlugin,
            WinGamePlugin,
            InstructionScreenPlugin,
        ))
        .run();
}
