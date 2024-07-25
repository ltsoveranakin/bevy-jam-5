use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::levels::data::LevelData;
use crate::levels::level_loader::LevelDataHandleRes;
use crate::math::tile_pos_to_world_pos;

const PLAYER_SPEED: f32 = 60.;
const JUMP_POWER: f32 = 300.;
const MID_AIR_SPEED_DEGREDATION: f32 = 60.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDeathEvent>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    check_player_on_ground,
                    move_player.after(check_player_on_ground),
                    check_player_death,
                    respawn_player_death,
                ),
            );
    }
}

#[derive(Component)]
pub struct Player {
    on_ground: bool,
    collider: Collider,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Player {
                on_ground: false,
                collider: Collider::capsule_y(3., 5.8),
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
            parent.spawn(SpriteBundle {
                texture: asset_server.load("image/character/snowman.png"),
                transform: Transform::from_xyz(0., 6., 1.),
                ..default()
            });
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
        if keys.pressed(KeyCode::KeyA) {
            desired_x_velocity -= PLAYER_SPEED;
        }

        if keys.pressed(KeyCode::KeyD) {
            desired_x_velocity += PLAYER_SPEED;
        }

        if desired_x_velocity != 0. {
            velocity.linvel.x = desired_x_velocity;
        }
    } else if velocity.linvel.x > 0. {
        if keys.pressed(KeyCode::KeyA) {
            velocity.linvel.x -= MID_AIR_SPEED_DEGREDATION * time.delta_seconds();
        }
    } else if keys.pressed(KeyCode::KeyD) {
        velocity.linvel.x += MID_AIR_SPEED_DEGREDATION * time.delta_seconds();
    }
}

fn check_player_on_ground(
    mut player_query: Query<(Entity, &mut Player, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    let (entity, mut player, transform) = player_query.single_mut();

    // shape cast to check if on ground

    let cast_start = transform.translation.truncate() - Vec2::new(0., 0.2);
    let shape_rotation = 0.;
    let cast_direction = Vec2::NEG_Y;
    let collider_shape = &player.collider;
    let cast_options = default();
    let query_filter = QueryFilter::default().exclude_collider(entity);

    if let Some((_entity, shape_hit)) = rapier_context.cast_shape(
        cast_start,
        shape_rotation,
        cast_direction,
        collider_shape,
        cast_options,
        query_filter,
    ) {
        player.on_ground = shape_hit.time_of_impact == 0.;
    }
}

#[derive(Event, Default)]
pub struct PlayerDeathEvent;

fn check_player_death(
    player_query: Query<&Transform, With<Player>>,
    mut player_death_ev: EventWriter<PlayerDeathEvent>,
) {
    let transform = player_query.single();

    if transform.translation.y < -20. {
        player_death_ev.send_default();
    }
}

fn respawn_player_death(
    mut player_query: Query<&mut Transform, With<Player>>,
    level_data_handle: Res<LevelDataHandleRes>,
    level_data_assets: Res<Assets<LevelData>>,
    mut player_death_ev: EventReader<PlayerDeathEvent>,
) {
    let mut player_transform = player_query.single_mut();
    if player_death_ev.read().next().is_some() {
        if let Some(handle) = level_data_handle.0.clone() {
            let level_data = level_data_assets.get(handle.id()).unwrap();
            player_transform.translation = tile_pos_to_world_pos(
                level_data.spawn_location.into(),
                player_transform.translation.z,
            );
        }
    }
}
