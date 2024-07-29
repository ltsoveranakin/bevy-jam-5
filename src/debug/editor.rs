use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;

use bevy::prelude::*;
use bevy::utils::AHasher;
use bevy_ecs_tilemap::prelude::*;

use crate::debug::DebugUpdateSet;
use crate::levels::data::{LevelData, LocationData, OverlayData, TileData, TileTypeData};
use crate::levels::level_loader::LevelDataHandleRes;
use crate::levels::{CurrentLevel, MainMap, OverlayMap};
use crate::math::world_pos_to_tile_pos;

pub struct DebugEditorPlugin;

impl Plugin for DebugEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EditorState>()
            .configure_sets(Update, EditorUpdateSet.run_if(in_state(EditorState::On)))
            .init_resource::<CurrentEditorTile>()
            .init_resource::<MousePosition>()
            .add_systems(
                Update,
                (
                    toggle_editor_mode.in_set(DebugUpdateSet),
                    (
                        change_editor_tile,
                        editor_click_tile,
                        update_cursor_pos,
                        save_current_tile_map,
                    )
                        .in_set(EditorUpdateSet),
                ),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct EditorUpdateSet;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum EditorState {
    #[default]
    Off,
    On,
}

#[derive(Resource, Debug)]
enum CurrentEditorTile {
    Base(TileTypeData),
    Overlay(OverlayData),
}

impl Default for CurrentEditorTile {
    fn default() -> Self {
        Self::Base(TileTypeData::Dirt)
    }
}

fn toggle_editor_mode(
    keys: Res<ButtonInput<KeyCode>>,
    editor_state: Res<State<EditorState>>,
    mut next_editor_state: ResMut<NextState<EditorState>>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        if *editor_state == EditorState::Off {
            next_editor_state.set(EditorState::On);
            println!("Editor ON");
            println!("Press 'n' to switch to next tile");
            println!("Press 'k' to save to file in current directory");
            println!("Left click to set tile");
            println!("Right click to delete tile in current scope (overlay vs. regular tile)");
            println!("File needs to be manually copied to assets/level");
        } else {
            next_editor_state.set(EditorState::Off);
            println!("Editor OFF");
        }
    }
}

fn change_editor_tile(
    keys: Res<ButtonInput<KeyCode>>,
    mut current_tile: ResMut<CurrentEditorTile>,
) {
    if keys.just_pressed(KeyCode::KeyN) {
        let switch_to = match &*current_tile {
            CurrentEditorTile::Base(tile) => match tile {
                TileTypeData::Dirt => CurrentEditorTile::Base(TileTypeData::Stone),
                TileTypeData::Stone => CurrentEditorTile::Base(TileTypeData::Water),
                TileTypeData::Water => CurrentEditorTile::Overlay(OverlayData::Grass),
            },

            CurrentEditorTile::Overlay(overlay) => match overlay {
                OverlayData::Grass => CurrentEditorTile::Base(TileTypeData::Dirt),
            },
        };

        *current_tile = switch_to;

        println!("Current Tile: {:?}", &*current_tile);
    }
}

#[derive(Resource, Default)]
struct MousePosition {
    world_pos: Vec2,
    window_pos: Vec2,
}

fn update_cursor_pos(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_ev: EventReader<CursorMoved>,
    mut mouse_pos: ResMut<MousePosition>,
) {
    let (global_transform, camera) = camera_query.single();
    for cursor_moved in cursor_moved_ev.read() {
        if let Some(pos) = camera.viewport_to_world_2d(global_transform, cursor_moved.position) {
            mouse_pos.world_pos = pos;
            mouse_pos.window_pos = cursor_moved.position;
        }
    }
}

type EntityStorage<'a> = (Entity, &'a mut TileStorage);

fn editor_click_tile(
    mut commands: Commands,
    mut main_map_query: Query<EntityStorage, (With<MainMap>, Without<OverlayMap>)>,
    mut overlay_map_query: Query<EntityStorage, (With<OverlayMap>, Without<MainMap>)>,
    tile_query: Query<&mut TileTextureIndex>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_position: Res<MousePosition>,
    current_editor_tile: Res<CurrentEditorTile>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let tile_pos = world_pos_to_tile_pos(mouse_position.world_pos.extend(0.)).into();

        match &*current_editor_tile {
            CurrentEditorTile::Base(base) => {
                let (main_map_entity, mut main_map_storage) = main_map_query.single_mut();
                let texture_index = base.texture_index();

                set_tile_map_tile(
                    commands.reborrow(),
                    &mut main_map_storage,
                    tile_pos,
                    tile_query,
                    texture_index,
                    main_map_entity,
                );
            }
            CurrentEditorTile::Overlay(overlay) => {
                let (overlay_map_entity, mut overlay_map_storage) = overlay_map_query.single_mut();
                let texture_index = overlay.texture_index();

                set_tile_map_tile(
                    commands.reborrow(),
                    &mut overlay_map_storage,
                    tile_pos,
                    tile_query,
                    texture_index,
                    overlay_map_entity,
                );
            }
        }
    } else if mouse_button.just_pressed(MouseButton::Right) {
        let tile_pos = world_pos_to_tile_pos(mouse_position.world_pos.extend(0.)).into();

        match &*current_editor_tile {
            CurrentEditorTile::Base(_) => {
                let (_, mut main_map_storage) = main_map_query.single_mut();

                clear_tile(commands.reborrow(), &mut main_map_storage, tile_pos);
            }
            CurrentEditorTile::Overlay(_) => {
                let (_, mut overlay_map_storage) = overlay_map_query.single_mut();

                clear_tile(commands.reborrow(), &mut overlay_map_storage, tile_pos);
            }
        }
    }
}

fn set_tile_map_tile(
    mut commands: Commands,
    tile_storage: &mut TileStorage,
    tile_pos: TilePos,
    mut tile_query: Query<&mut TileTextureIndex>,
    texture_index: u32,
    tile_map_entity: Entity,
) {
    if let Some(tile_entity) = tile_storage.get(&tile_pos) {
        let mut tile_texture_index = tile_query.get_mut(tile_entity).unwrap();
        tile_texture_index.0 = texture_index;
    } else {
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tile_map_entity),
                texture_index: TileTextureIndex(texture_index),
                ..default()
            })
            .id();

        tile_storage.set(&tile_pos, tile_entity);
    }
}

fn clear_tile(mut commands: Commands, map_storage: &mut TileStorage, tile_pos: TilePos) {
    if let Some(tile_entity) = map_storage.get(&tile_pos) {
        if let Some(entity_commands) = commands.get_entity(tile_entity) {
            entity_commands.despawn_recursive();
            map_storage.remove(&tile_pos);
        }
    }
}

fn save_current_tile_map(
    main_map_query: Query<&TileStorage, With<MainMap>>,
    overlay_map_query: Query<&TileStorage, With<OverlayMap>>,
    tile_query: Query<(&TilePos, &TileTextureIndex)>,
    level_data_handle: Res<LevelDataHandleRes>,
    current_level: Res<CurrentLevel>,
    level_data_assets: Res<Assets<LevelData>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyK) {
        let tile_storage = main_map_query.single();
        let overlay_map_storage = overlay_map_query.single();

        let spawn_location = if let Some(handle) = level_data_handle.0.clone() {
            let level_data = level_data_assets.get(handle.id()).unwrap();
            level_data.spawn_location
        } else {
            LocationData::new(0, 0)
        };

        let mut level_data = LevelData {
            spawn_location,
            tiles: vec![],
        };

        for tile_entity in tile_storage.iter().flatten() {
            let (tile_pos, texture_index) = tile_query.get(*tile_entity).unwrap();

            let over = if let Some(overlay_entity) = overlay_map_storage.get(tile_pos) {
                let (_, texture_index) = tile_query.get(overlay_entity).unwrap();

                Some(OverlayData::from_texture_index(texture_index.0))
            } else {
                None
            };

            level_data.tiles.push(TileData {
                tile_type: TileTypeData::from_texture_index(texture_index.0),
                off: LocationData {
                    x: tile_pos.x,
                    y: tile_pos.y,
                },
                over,
            });
        }

        let json_str = serde_json::to_string(&level_data).unwrap();

        let file_out_path = &format!("assets/level/level{}.lvl.json", current_level.0);

        let mut file = File::create(file_out_path).unwrap();

        file.write_all(json_str.as_bytes())
            .expect("Unable to write to file");

        println!("Wrote data to file at {}", file_out_path);
    }
}
