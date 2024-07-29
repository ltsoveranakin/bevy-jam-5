use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

use crate::day_night::DayNightState;

const FADE_TIME: f32 = 4.;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_music)
            .add_systems(Update, fade_audio)
            .add_systems(OnEnter(DayNightState::Night), night_music)
            .add_systems(OnEnter(DayNightState::Day), day_music);
    }
}

#[derive(Component)]
struct NightAudio;

#[derive(Component)]
struct DayAudio;

#[derive(Component)]
enum FadeAudioDirection {
    Increasing,
    Decreasing,
}

impl FadeAudioDirection {
    fn get_fade_multiplier(&self) -> f32 {
        match self {
            FadeAudioDirection::Increasing => 1.,
            FadeAudioDirection::Decreasing => -1.,
        }
    }
}

fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let day_handle = asset_server.load::<AudioSource>("audio/music/day.mp3");
    let night_handle = asset_server.load::<AudioSource>("audio/music/night.mp3");

    commands.spawn((
        AudioBundle {
            source: day_handle,
            settings: PlaybackSettings {
                volume: Volume::ZERO,
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
        DayAudio,
        FadeAudioDirection::Increasing,
    ));

    commands.spawn((
        AudioBundle {
            source: night_handle,
            settings: PlaybackSettings {
                volume: Volume::ZERO,
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
        NightAudio,
    ));
}

fn night_music(
    mut commands: Commands,
    day_audio_query: Query<Entity, With<DayAudio>>,
    night_audio_query: Query<Entity, With<NightAudio>>,
) {
    if let Ok(night_audio_entity) = night_audio_query.get_single() {
        commands
            .entity(night_audio_entity)
            .insert(FadeAudioDirection::Increasing);
    }

    if let Ok(day_audio_entity) = day_audio_query.get_single() {
        commands
            .entity(day_audio_entity)
            .insert(FadeAudioDirection::Decreasing);
    }
}

fn day_music(
    mut commands: Commands,
    night_audio_query: Query<Entity, With<NightAudio>>,
    day_audio_query: Query<Entity, With<DayAudio>>,
) {
    if let Ok(night_audio_entity) = night_audio_query.get_single() {
        commands
            .entity(night_audio_entity)
            .insert(FadeAudioDirection::Decreasing);
    }

    if let Ok(day_audio_entity) = day_audio_query.get_single() {
        commands
            .entity(day_audio_entity)
            .insert(FadeAudioDirection::Increasing);
    }
}

fn fade_audio(
    mut commands: Commands,
    audio_query: Query<(Entity, &AudioSink, &FadeAudioDirection)>,
    time: Res<Time>,
) {
    for (entity, audio_sink, fade_audio_direction) in audio_query.iter() {
        let volume = (audio_sink.volume()
            + (time.delta_seconds() / FADE_TIME * fade_audio_direction.get_fade_multiplier()))
        .clamp(0., 1.);

        audio_sink.set_volume(volume);

        match fade_audio_direction {
            FadeAudioDirection::Increasing => {
                // println!("inc {}");
                audio_sink.play();
                if volume >= 1. {
                    commands.entity(entity).remove::<FadeAudioDirection>();
                }
            }
            FadeAudioDirection::Decreasing => {
                // println!("dec {}");
                if volume <= 0. {
                    audio_sink.pause();
                    commands.entity(entity).remove::<FadeAudioDirection>();
                }
            }
        }
    }
}
