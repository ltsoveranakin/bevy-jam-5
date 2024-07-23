use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Player,
            InheritedVisibility::default(),
            Collider::ball(5.),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController::default(),
            TransformBundle::default(),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset_server.load("image/character/snowman.png"),
                ..default()
            });
        });
}
