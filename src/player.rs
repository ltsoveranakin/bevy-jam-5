use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 10.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Player,
            InheritedVisibility::default(),
            Collider::ball(16.),
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

fn move_player(
    mut player_query: Query<&mut KinematicCharacterController>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut character_controller = player_query.single_mut();

    let mut translation = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        // translation.y += PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::KeyA) {
        translation.x -= PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::KeyD) {
        translation.x += PLAYER_SPEED;
    }

    character_controller.translation = if translation == Vec2::ZERO {
        None
    } else {
        Some(translation)
    };
}
