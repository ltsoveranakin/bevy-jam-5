use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::geometry::ShapeCastOptions;
use bevy_rapier2d::pipeline::QueryFilter;
use bevy_rapier2d::plugin::RapierContext;

use crate::player::Player;

const PLAYER_MAX_VELOCITY: f32 = 80.;

/// The acceleration of the player, the player will accelerate to [`PLAYER_MAX_VELOCITY`] in [`PLAYER_MAX_VELOCITY`]/[`ACCELERATION`] seconds
const ACCELERATION: f32 = 320.;
const JUMP_POWER: f32 = 300.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_player_on_ground,
                (player_move, player_jump).in_set(ControlPlayerSet),
                apply_acceleration.after(ControlPlayerSet),
            ),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ControlPlayerSet;

fn player_move(mut player_query: Query<&mut Player>, keys: Res<ButtonInput<KeyCode>>) {
    let mut player = player_query.single_mut();

    let mut desired_x_acceleration = 0.;

    if keys.pressed(KeyCode::KeyA) {
        desired_x_acceleration -= ACCELERATION;
    }
    if keys.pressed(KeyCode::KeyD) {
        desired_x_acceleration += ACCELERATION;
    }

    player.x_acceleration = desired_x_acceleration;
}

fn player_jump(mut player_query: Query<(&Player, &mut Velocity)>, keys: Res<ButtonInput<KeyCode>>) {
    let (player, mut velocity) = player_query.single_mut();

    if player.on_ground && (keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::Space)) {
        velocity.linvel.y = JUMP_POWER;
    }
}

fn apply_acceleration(mut player_query: Query<(&mut Player, &mut Velocity)>, time: Res<Time>) {
    let (mut player, mut velocity) = player_query.single_mut();

    if player.x_acceleration != 0. {
        player.x_velocity += player.x_acceleration * time.delta_seconds();

        player.x_velocity = player
            .x_velocity
            .clamp(-PLAYER_MAX_VELOCITY, PLAYER_MAX_VELOCITY);

        velocity.linvel.x = player.x_velocity;
    } else {
        player.x_velocity = velocity.linvel.x;
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
    let cast_options = ShapeCastOptions::with_max_time_of_impact(32.);
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
