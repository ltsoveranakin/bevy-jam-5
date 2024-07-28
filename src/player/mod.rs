use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::day_night::{DayNightState, SetDayNightEvent};
use crate::levels::data::LevelData;
use crate::levels::level_loader::LevelDataHandleRes;
use crate::levels::LoadNextLevelEvent;
use crate::math::tile_pos_to_world_pos;
use crate::player::melting::{MeltingPlugin, MeltStage, TimeUnderSun};

mod melting;

const PLAYER_MAX_SPEED: f32 = 80.;
const JUMP_POWER: f32 = 300.;
const MID_AIR_SPEED_DEGRADATION: f32 = 100.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDeathEvent>()
            .add_event::<PlayerFinishLevelEvent>()
            .add_event::<RespawnPlayerEvent>()
            .add_plugins(MeltingPlugin)
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    check_player_on_ground,
                    move_player.after(check_player_on_ground),
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

#[derive(Component)]
pub struct Player {
    on_ground: bool,
    collider: Collider,
    melt_stage: MeltStage,
}

#[derive(Component)]
pub struct PlayerSprite;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Player {
                on_ground: false,
                collider: Collider::capsule_y(2.5, 5.5),
                melt_stage: MeltStage::None,
            },
            InheritedVisibility::default(),
            Collider::capsule_y(3., 6.),
            RigidBody::Dynamic,
            Velocity::default(),
            KinematicCharacterController::default(),
            KinematicCharacterControllerOutput::default(),
            TransformBundle::default(),
            LockedAxes::ROTATION_LOCKED,
            Ccd::enabled(),
            Friction::new(0.2),
            Restitution::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("image/character/snowman.png"),
                    transform: Transform::from_xyz(0., 6., 2.),
                    sprite: Sprite {
                        rect: Some(Rect::from_center_half_size(
                            MeltStage::None.get_sprite_offset(),
                            Vec2::splat(16.),
                        )),
                        ..default()
                    },
                    ..default()
                },
                PlayerSprite,
            ));
        });
}

fn move_player(
    mut player_query: Query<(&Player, &mut Velocity)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut velocity) = player_query.single_mut();

    let mut desired_x_velocity = 0.;

    if player.on_ground && (keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::Space)) {
        velocity.linvel.y = JUMP_POWER;
    }

    if player.on_ground {
        // println!("grounded - {}", time.elapsed_seconds());
        if keys.pressed(KeyCode::KeyA) {
            desired_x_velocity -= PLAYER_MAX_SPEED;
        }

        if keys.pressed(KeyCode::KeyD) {
            desired_x_velocity += PLAYER_MAX_SPEED;
        }

        if desired_x_velocity != 0. {
            velocity.linvel.x = desired_x_velocity * player.melt_stage.get_speed_multiplier();
        }
    } else if velocity.linvel.x > 0. {
        if keys.pressed(KeyCode::KeyA) {
            velocity.linvel.x -= MID_AIR_SPEED_DEGRADATION * time.delta_seconds();
        }
    } else if keys.pressed(KeyCode::KeyD) {
        velocity.linvel.x += MID_AIR_SPEED_DEGRADATION * time.delta_seconds();
    }
}

fn check_player_on_ground(
    mut player_query: Query<(Entity, &mut Player, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    let (entity, mut player, transform) = player_query.single_mut();

    // shape cast to check if on ground

    let cast_start = transform.translation.truncate() - Vec2::new(0., 1.);
    let shape_rotation = 0.;
    let cast_direction = Vec2::NEG_Y;
    let collider_shape = &player.collider;
    let cast_options = ShapeCastOptions::default();
    // let cast_options = ShapeCastOptions::with_max_time_of_impact(1.);
    let query_filter = QueryFilter::default().exclude_collider(entity);

    if let Some((_entity, shape_hit)) = rapier_context.cast_shape(
        cast_start,
        shape_rotation,
        cast_direction,
        collider_shape,
        cast_options,
        query_filter,
    ) {
        println!("toi: {}", shape_hit.time_of_impact);
        player.on_ground = shape_hit.time_of_impact == 0.;
    }
}

#[derive(Event, Default)]
pub struct PlayerDeathEvent;

#[derive(Event, Default)]
pub struct PlayerFinishLevelEvent;

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

#[derive(Event, Default)]
pub struct RespawnPlayerEvent;

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
