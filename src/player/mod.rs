use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::melting::{MeltingPlugin, MeltStage};
use crate::player::movement::MovementPlugin;
use crate::player::respawn::RespawnPlugin;

mod melting;
mod movement;
pub mod respawn;

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
