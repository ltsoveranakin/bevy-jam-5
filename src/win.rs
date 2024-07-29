use bevy::prelude::*;

use crate::levels::LoadLevelEvent;

pub struct WinGamePlugin;

impl Plugin for WinGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WinGameEvent>()
            .add_systems(Update, check_win_game);
    }
}

#[derive(Event, Default)]
pub struct WinGameEvent;

fn check_win_game(
    mut win_game_event: EventReader<WinGameEvent>,
    mut load_level_event: EventWriter<LoadLevelEvent>,
) {
    if win_game_event.read().next().is_some() {
        load_level_event.send(LoadLevelEvent(999));
    }
}
