use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::camera::NIGHT_COLOR;
use crate::levels::data::LocationData;
use crate::levels::{TileLevelLoadedEvent, TILE_MAP_SIZE, TILE_SIZE};
use crate::math::tile_pos_to_world_pos_2d;
use crate::z_indecies::SHADOW_Z_INDEX;

const SHADOW_ALPHA: f32 = 0.4;

pub struct NightPlugin;

impl Plugin for NightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShadowMaterial>()
            .add_systems(Startup, create_shadow_material)
            .add_systems(Update, create_shadows);
    }
}

#[derive(Resource, Default)]
struct ShadowMaterial(Handle<ColorMaterial>);

fn create_shadow_material(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shadow_material: ResMut<ShadowMaterial>,
) {
    let mut shadow_color = NIGHT_COLOR;
    shadow_color.set_alpha(SHADOW_ALPHA);

    shadow_material.0 = materials.add(shadow_color);
}

fn create_shadows(
    mut commands: Commands,
    mut tile_level_loaded_event: EventReader<TileLevelLoadedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    shadow_material: Res<ShadowMaterial>,
) {
    if let Some(tile_level_loaded) = tile_level_loaded_event.read().next() {
        let tile_map = &tile_level_loaded.level_data_map;

        for x in 0..TILE_MAP_SIZE {
            // start at index of 1, because no need to cast shadow where camera does not see (beyond extents of game)
            for y in 1..TILE_MAP_SIZE {
                if !tile_map.contains_key(&LocationData::new(x, y)) {
                    continue;
                }

                let mut shadow_len = 0;
                let mut cast_y = y;

                while cast_y > 0 {
                    cast_y -= 1;

                    if tile_map.contains_key(&LocationData::new(x, cast_y)) {
                        break;
                    }

                    shadow_len += 1;
                }

                if shadow_len == 0 {
                    continue;
                }

                let start_pos = UVec2::new(x, y - 1);
                let end_pos = start_pos - UVec2::new(0, shadow_len - 1);

                let world_start_pos = tile_pos_to_world_pos_2d(start_pos);
                let world_end_pos = tile_pos_to_world_pos_2d(end_pos);

                println!("start: {}, end: {}", start_pos, end_pos);

                let midpoint = ((world_start_pos + world_end_pos) / 2.).extend(SHADOW_Z_INDEX);

                let shadow_half_size = Vec2::new(TILE_SIZE, shadow_len as f32 * TILE_SIZE);

                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::from_size(shadow_half_size)).into(),
                    material: shadow_material.0.clone(),
                    transform: Transform::from_translation(midpoint),
                    ..default()
                });
            }
        }
    }
}
