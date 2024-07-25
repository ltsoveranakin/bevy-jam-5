use bevy::prelude::*;

use crate::debug::DebugUpdateSet;

const CAMERA_MOVE_SPEED: f32 = 50.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera.in_set(DebugUpdateSet));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(152., 112., 0.),
            scale: Vec3::splat(0.4),
            ..default()
        },
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb_u8(51, 141, 242)),
            ..default()
        },
        ..default()
    });
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
