use bevy::prelude::*;

use crate::z_indices::INSTRUCTION_SCREEN_Z_INDEX;

pub struct InstructionScreenPlugin;

impl Plugin for InstructionScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(Update, GameRunSet.run_if(in_state(GameState::Play)))
            .add_systems(Startup, spawn_instructions_screen)
            .add_systems(
                Update,
                input_check_game_state_ready.run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnEnter(GameState::Play), hide_instructions);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameRunSet;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Play,
}

#[derive(Component)]
struct InstructionScreenSprite;

fn spawn_instructions_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("image/screen/instruction.jpg"),
            transform: Transform::from_xyz(0., 0., INSTRUCTION_SCREEN_Z_INDEX),
            ..default()
        },
        InstructionScreenSprite,
    ));
}

fn input_check_game_state_ready(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keys.get_pressed().len() > 0 || mouse_button.get_pressed().len() > 0 {
        next_game_state.set(GameState::Play);
    }
}

fn hide_instructions(
    mut commands: Commands,
    instruction_screen_sprite_query: Query<Entity, With<InstructionScreenSprite>>,
) {
    let entity = instruction_screen_sprite_query.single();
    commands.entity(entity).despawn();
}
