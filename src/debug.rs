use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_debug_graphics);
    }
}

fn toggle_debug_graphics(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if keys.just_pressed(KeyCode::Backquote) {
        debug_render_context.enabled = !debug_render_context.enabled;
    }
}
