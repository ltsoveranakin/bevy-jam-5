use bevy::prelude::*;

use crate::camera::{DAY_COLOR, NIGHT_COLOR};

pub struct DayNightPlugin;

impl Plugin for DayNightPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DayNightState>()
            .add_event::<SetDayNightEvent>()
            .add_systems(Update, (set_day_night_cycle));
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DayNightState {
    #[default]
    Day,
    Night,
}

#[derive(Event)]
pub struct SetDayNightEvent(pub DayNightState);

fn set_day_night_cycle(
    mut camera_query: Query<&mut Camera>,
    mut set_day_night_ev: EventReader<SetDayNightEvent>,
    mut day_night_cycle_next: ResMut<NextState<DayNightState>>,
) {
    let mut camera = camera_query.single_mut();

    if let Some(set_day_night) = set_day_night_ev.read().next() {
        match set_day_night.0 {
            DayNightState::Day => {
                camera.clear_color = ClearColorConfig::Custom(DAY_COLOR);
            }
            DayNightState::Night => {
                camera.clear_color = ClearColorConfig::Custom(NIGHT_COLOR);
            }
        }
        day_night_cycle_next.set(set_day_night.0);
    }
}
