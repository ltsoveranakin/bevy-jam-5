use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod level_loader;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, set_current_level);
    }
}

#[derive(Event)]
pub struct LoadLevelEvent(i32);

fn setup() {}
