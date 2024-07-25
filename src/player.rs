use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 0.5;
const JUMP_POWER: f32 = 2.;
const PLAYER_GRAVITY: f32 = 10.;
const PLAYER_MAX_GRAVITY: f32 = 100.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                player_gravity,
                move_player.after(player_gravity),
                apply_player_velocity.after(move_player),
            ),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct PlayerVelocity(Vec2);

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Player,
            PlayerVelocity(default()),
            InheritedVisibility::default(),
            Collider::capsule_y(3., 6.),
            RigidBody::KinematicVelocityBased,
            KinematicCharacterController::default(),
            KinematicCharacterControllerOutput::default(),
            TransformBundle::default(),
            LockedAxes::ROTATION_LOCKED,
            Ccd::enabled(),
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
    mut player_query: Query<(&mut PlayerVelocity, &KinematicCharacterControllerOutput)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (mut player_velocity, controller_output) = player_query.single_mut();

    let mut translation = 0.;

    if controller_output.grounded && (keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::Space)) {
        player_velocity.0.y = JUMP_POWER;
    }

    if keys.pressed(KeyCode::KeyA) {
        translation -= PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::KeyD) {
        translation += PLAYER_SPEED;
    }

    player_velocity.0.x = translation;
}

fn player_gravity(
    mut player_query: Query<(&mut PlayerVelocity, &KinematicCharacterControllerOutput)>,
    time: Res<Time>,
) {
    let (mut player_velocity, controller_output) = player_query.single_mut();

    if !controller_output.grounded {
        player_velocity.0.y -= PLAYER_GRAVITY * time.delta_seconds();

        if player_velocity.0.y < -PLAYER_MAX_GRAVITY {
            player_velocity.0.y = -PLAYER_MAX_GRAVITY;
        }
    } else {
        player_velocity.0.y = 0.;
    }
}

fn apply_player_velocity(
    mut player_query: Query<(&mut KinematicCharacterController, &PlayerVelocity)>,
) {
    let (mut character_controller, player_velocity) = player_query.single_mut();

    if player_velocity.0 != Vec2::ZERO {
        character_controller.translation = Some(player_velocity.0);
    }
}
