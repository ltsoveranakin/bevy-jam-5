use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::geometry::ShapeCastOptions;
use bevy_rapier2d::pipeline::QueryFilter;
use bevy_rapier2d::plugin::RapierContext;

use crate::player::Player;

const PLAYER_MAX_SPEED: f32 = 80.;
const JUMP_POWER: f32 = 300.;
const MID_AIR_SPEED_DEGRADATION: f32 = 100.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_player_on_ground,
                move_player.after(check_player_on_ground),
            ),
        );
    }
}

fn move_player(
    mut player_query: Query<(&Player, &mut Velocity)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut velocity) = player_query.single_mut();

    let mut desired_x_velocity = 0.;

    let air_mul = if player.on_ground {
        if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::Space) {
            velocity.linvel.y = JUMP_POWER;
        }

        1.
    } else {
        0.5
    };

    if keys.pressed(KeyCode::KeyA) {
        desired_x_velocity -= PLAYER_MAX_SPEED;
    }

    if keys.pressed(KeyCode::KeyD) {
        desired_x_velocity += PLAYER_MAX_SPEED;
    }

    if desired_x_velocity != 0. {
        velocity.linvel.x = desired_x_velocity * player.melt_stage.get_speed_multiplier() * air_mul;
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
