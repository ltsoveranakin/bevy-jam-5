use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::melting::{MeltingPlugin, MeltStage};
use crate::player::movement::MovementPlugin;
use crate::player::respawn::RespawnPlugin;
use crate::z_indices::PLAYER_Z_INDEX;

mod melting;
mod movement;
pub mod respawn;

const CAST_COLLIDER_SCALE: f32 = 0.9;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MeltingPlugin, MovementPlugin, RespawnPlugin))
            .add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player {
    on_ground: bool,
    on_wall: bool,
    cast_collider: Collider,
    melt_stage: MeltStage,
    x_acceleration: f32,
    x_velocity: f32,
}

#[derive(Component)]
pub struct PlayerSprite;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let melt_stage = MeltStage::None;
    let collider_dimensions = melt_stage.get_collider_dimensions();
    let cast_collider_dimensions = melt_stage.get_cast_collider_dimensions();

    let player = Player {
        on_ground: false,
        on_wall: false,
        cast_collider: Collider::capsule_y(cast_collider_dimensions.x, cast_collider_dimensions.y),
        melt_stage,
        x_acceleration: 0.,
        x_velocity: 0.,
    };

    commands
        .spawn((
            player,
            InheritedVisibility::default(),
            Collider::capsule_y(collider_dimensions.x, collider_dimensions.y),
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
                    transform: Transform::from_translation(
                        melt_stage.get_sprite_offset().extend(PLAYER_Z_INDEX),
                    ),
                    sprite: Sprite {
                        rect: Some(Rect::from_center_half_size(
                            melt_stage.get_tile_sprite_offset(),
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
