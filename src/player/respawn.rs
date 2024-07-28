use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::day_night::{DayNightState, SetDayNightEvent};
use crate::levels::data::LevelData;
use crate::levels::level_loader::LevelDataHandleRes;
use crate::levels::LoadNextLevelEvent;
use crate::math::tile_pos_to_world_pos;
use crate::player::{Player, PlayerSprite};
use crate::player::melting::{MeltStage, TimeUnderSun};

pub struct RespawnPlugin;

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDeathEvent>()
            .add_event::<PlayerFinishLevelEvent>()
            .add_event::<RespawnPlayerEvent>()
            .add_systems(
                Update,
                (
                    (
                        check_player_out_of_bounds,
                        respawn_player_death.after(check_player_out_of_bounds),
                        respawn_player_finish_level.after(check_player_out_of_bounds),
                    )
                        .in_set(CheckPlayerForRespawn),
                    respawn_player.after(CheckPlayerForRespawn),
                    key_respawn,
                ),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CheckPlayerForRespawn;

#[derive(Event, Default)]
pub struct PlayerDeathEvent;

#[derive(Event, Default)]
pub struct PlayerFinishLevelEvent;

#[derive(Event, Default)]
pub struct RespawnPlayerEvent;

fn check_player_out_of_bounds(
    player_query: Query<&Transform, With<Player>>,
    mut player_death: EventWriter<PlayerDeathEvent>,
    mut player_finish_level: EventWriter<PlayerFinishLevelEvent>,
) {
    let transform = player_query.single();

    if transform.translation.y < -20. {
        player_death.send_default();
    }

    if transform.translation.x > 320. {
        player_finish_level.send_default();
        println!("fin level");
    }
}

fn respawn_player_death(
    mut player_query: Query<&mut Player>,
    mut player_death_ev: EventReader<PlayerDeathEvent>,
    mut respawn_player: EventWriter<RespawnPlayerEvent>,
    mut set_day_night: EventWriter<SetDayNightEvent>,
) {
    let mut player = player_query.single_mut();
    if player_death_ev.read().next().is_some() {
        println!("death");
        respawn_player.send_default();
        set_day_night.send(SetDayNightEvent(DayNightState::Day));
        player.melt_stage = MeltStage::None;
    }
}

fn respawn_player_finish_level(
    mut player_query: Query<&mut Player>,
    mut player_finish_level_event: EventReader<PlayerFinishLevelEvent>,
    mut respawn_player: EventWriter<RespawnPlayerEvent>,
    mut set_day_night: EventWriter<SetDayNightEvent>,
    mut load_next_level: EventWriter<LoadNextLevelEvent>,
    day_night_state: Res<State<DayNightState>>,
) {
    let mut player = player_query.single_mut();
    if player_finish_level_event.read().next().is_some() {
        respawn_player.send_default();
        println!("level fin");
        match day_night_state.get() {
            DayNightState::Day => {
                println!("replaying at night");
                // replay same level, but at night
                set_day_night.send(SetDayNightEvent(DayNightState::Night));
            }
            DayNightState::Night => {
                println!("moving to next level");
                // next level
                set_day_night.send(SetDayNightEvent(DayNightState::Day));
                player.melt_stage = MeltStage::None;
                load_next_level.send_default();
            }
        }
    }
}

fn respawn_player(
    mut player_query: Query<(&mut Transform, &mut Velocity, &Player)>,
    mut player_sprite_query: Query<&mut Sprite, With<PlayerSprite>>,
    level_data_handle: Res<LevelDataHandleRes>,
    level_data_assets: Res<Assets<LevelData>>,
    mut respawn_player_ev: EventReader<RespawnPlayerEvent>,
    mut time_under_sun: ResMut<TimeUnderSun>,
) {
    let (mut transform, mut velocity, player) = player_query.single_mut();
    let mut sprite = player_sprite_query.single_mut();

    if respawn_player_ev.read().next().is_some() {
        if let Some(handle) = level_data_handle.0.clone() {
            let level_data = level_data_assets.get(handle.id()).unwrap();

            velocity.linvel = Vec2::ZERO;
            transform.translation =
                tile_pos_to_world_pos(level_data.spawn_location.into(), transform.translation.z);
            time_under_sun.reset();
            sprite.rect = Some(Rect::from_center_half_size(
                player.melt_stage.get_sprite_offset(),
                Vec2::splat(16.),
            ));
        }
    }
}

fn key_respawn(keys: Res<ButtonInput<KeyCode>>, mut player_death: EventWriter<PlayerDeathEvent>) {
    if keys.just_pressed(KeyCode::KeyR) {
        player_death.send_default();
    }
}
