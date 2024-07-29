use bevy::prelude::*;

use crate::win::WinGameEvent;
use crate::z_indices::TEXT_Z_INDEX;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartGameEvent>()
            .init_resource::<StartTime>()
            .add_systems(Update, (start_timer, end_timer));
    }
}

#[derive(Event, Default)]
pub struct StartGameEvent;

#[derive(Resource, Default)]
struct StartTime(f32);

fn start_timer(
    mut start_game_event: EventReader<StartGameEvent>,
    mut start_time: ResMut<StartTime>,
    time: Res<Time>,
) {
    if start_game_event.read().next().is_some() {
        start_time.0 = time.elapsed_seconds();
    }
}

fn end_timer(
    mut commands: Commands,
    mut win_game_event: EventReader<WinGameEvent>,
    start_time: Res<StartTime>,
    time: Res<Time>,
) {
    if win_game_event.read().next().is_some() {
        let end_time = time.elapsed_seconds();
        let total_time = end_time - start_time.0;

        let time_rounded = (total_time * 10.).round() / 10.;

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("Completed in {} seconds!", time_rounded),
                TextStyle {
                    color: Color::srgb_u8(235, 64, 52),
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3::new(0., 180., TEXT_Z_INDEX),
                scale: Vec3::splat(0.35),
                ..default()
            },
            ..default()
        });
    }
}
