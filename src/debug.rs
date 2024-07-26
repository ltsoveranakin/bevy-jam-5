use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, DebugUpdateSet.run_if(in_state(DebugState::On)))
            .init_state::<DebugState>()
            .add_systems(Update, toggle_debug_state)
            .add_systems(OnEnter(DebugState::On), enable_debug_state)
            .add_systems(OnEnter(DebugState::Off), disable_debug_state);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DebugState {
    #[default]
    Off,
    On,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebugUpdateSet;

#[derive(Component)]
pub struct DebugVisibility;

fn toggle_debug_state(
    keys: Res<ButtonInput<KeyCode>>,
    debug_state: Res<State<DebugState>>,
    mut next_debug_state: ResMut<NextState<DebugState>>,
) {
    if keys.just_pressed(KeyCode::Backquote) {
        if *debug_state == DebugState::Off {
            next_debug_state.set(DebugState::On);
        } else {
            next_debug_state.set(DebugState::Off);
        }
    }
}

fn enable_debug_state(
    mut debug_render_context: ResMut<DebugRenderContext>,
    mut debug_visibility_query: Query<&mut Visibility, With<DebugVisibility>>,
) {
    debug_render_context.enabled = true;

    for mut debug_vis in debug_visibility_query.iter_mut() {
        *debug_vis = Visibility::Visible;
    }
}

fn disable_debug_state(
    mut debug_render_context: ResMut<DebugRenderContext>,
    mut debug_visibility_query: Query<&mut Visibility, With<DebugVisibility>>,
) {
    debug_render_context.enabled = false;

    for mut debug_vis in debug_visibility_query.iter_mut() {
        *debug_vis = Visibility::Hidden;
    }
}
