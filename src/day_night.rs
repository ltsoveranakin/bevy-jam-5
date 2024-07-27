use bevy::prelude::*;

use crate::camera::{DAY_COLOR, NIGHT_COLOR};
use crate::player::PlayerFinishLevelEvent;

pub struct DayNightPlugin;

impl Plugin for DayNightPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DayNightCycleState>()
            .add_systems(Update, change_day_night_cycle);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DayNightCycleState {
    #[default]
    Day,
    Night,
}

fn change_day_night_cycle(
    mut camera_query: Query<&mut Camera>,
    mut player_finish_level_event: EventReader<PlayerFinishLevelEvent>,
    day_night_cycle: Res<State<DayNightCycleState>>,
    mut day_night_cycle_next: ResMut<NextState<DayNightCycleState>>,
) {
    let mut camera = camera_query.single_mut();
    if player_finish_level_event.read().next().is_some() {
        match day_night_cycle.get() {
            DayNightCycleState::Day => {
                camera.clear_color = ClearColorConfig::Custom(NIGHT_COLOR);
                day_night_cycle_next.set(DayNightCycleState::Night);
            }
            DayNightCycleState::Night => {
                camera.clear_color = ClearColorConfig::Custom(DAY_COLOR);
                day_night_cycle_next.set(DayNightCycleState::Day);
            }
        }
    }
}
