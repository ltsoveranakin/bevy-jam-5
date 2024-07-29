use bevy::prelude::*;

use crate::debug::DebugUpdateSet;
use crate::instruction_screen::GameState;

pub const DAY_COLOR: Color = Color::srgb(0.31, 0.75, 0.88);
pub const NIGHT_COLOR: Color = Color::srgb(0., 0.11, 0.12);

const CAMERA_MOVE_SPEED: f32 = 50.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera.in_set(DebugUpdateSet))
            .add_systems(OnEnter(GameState::Play), game_start_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::splat(1.),
            ..default()
        },
        camera: Camera {
            clear_color: ClearColorConfig::Custom(DAY_COLOR),
            ..default()
        },
        ..default()
    });
}

fn game_start_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut transform = camera_query.single_mut();
    transform.translation = Vec3::new(152., 112., 0.);
    transform.scale = Vec3::splat(0.4);
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();

    let mut translation = Vec3::ZERO;

    let delta_speed = CAMERA_MOVE_SPEED * time.delta_seconds();

    if keys.pressed(KeyCode::ArrowUp) {
        translation.y += delta_speed;
    }

    if keys.pressed(KeyCode::ArrowDown) {
        translation.y -= delta_speed;
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        translation.x -= delta_speed;
    }

    if keys.pressed(KeyCode::ArrowRight) {
        translation.x += delta_speed;
    }

    camera_transform.translation += translation;

    if translation != Vec3::ZERO {
        println!("{:?}", camera_transform.translation);
    }
}
