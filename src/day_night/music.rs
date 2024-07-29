use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

use crate::day_night::DayNightState;

const FADE_TIME: f32 = 2.;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MusicHandles>()
            .add_systems(Startup, setup_music)
            .add_systems(Update, fade_audio)
            .add_systems(OnEnter(DayNightState::Night), night_music)
            .add_systems(OnEnter(DayNightState::Day), day_music);
    }
}

#[derive(Resource, Default)]
struct MusicHandles {
    night: Handle<AudioSource>,
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

fn setup_music(mut music_handles: ResMut<MusicHandles>, asset_server: Res<AssetServer>) {
    music_handles.night = asset_server.load("audio/music/night.mp3");
}

fn night_music(
    mut commands: Commands,
    day_audio_query: Query<Entity, With<DayAudio>>,
    music_handles: Res<MusicHandles>,
) {
    for entity in day_audio_query.iter() {
        commands
            .entity(entity)
            .insert(FadeAudioDirection::Decreasing);
    }

    commands.spawn((
        AudioBundle {
            source: music_handles.night.clone(),
            settings: PlaybackSettings {
                volume: Volume::ZERO,
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        NightAudio,
        FadeAudioDirection::Increasing,
    ));
}

fn day_music(mut commands: Commands, night_audio_query: Query<Entity, With<NightAudio>>) {
    for entity in night_audio_query.iter() {
        commands
            .entity(entity)
            .insert(FadeAudioDirection::Decreasing);
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
                if volume >= 1. {
                    commands.entity(entity).remove::<FadeAudioDirection>();
                }
            }
            FadeAudioDirection::Decreasing => {
                if volume <= 0. {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
