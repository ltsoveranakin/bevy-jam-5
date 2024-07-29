use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::day_night::shadow::DespawnShadowsEvent;
use crate::debug::{DebugState, DebugUpdateSet, DebugVisibility};
use crate::levels::data::{LevelData, LocationData, TileTypeData};
use crate::levels::level_loader::{LevelDataLoadedEvent, LevelLoaderPlugin};
use crate::math::tile_pos_to_world_pos;
use crate::player::respawn::RespawnPlayerEvent;
use crate::z_indecies::TILE_MAP_Z_INDEX;

pub mod data;
pub mod level_loader;

pub const TILE_MAP_SIZE: u32 = 32;
pub const TILE_MAP_SIZE_F32: f32 = TILE_MAP_SIZE as f32;
pub const TILE_SIZE: f32 = 16.;
pub const HALF_TILE_SIZE: f32 = 8.;

pub const MAX_LEVEL_INDEX: u32 = 3;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelLoaderPlugin)
            .add_event::<LoadLevelEvent>()
            .add_event::<LoadNextLevelEvent>()
            .add_event::<TileLevelLoadedEvent>()
            .init_resource::<CurrentLevel>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    level_data_ready,
                    receive_load_level,
                    receive_load_next_level,
                    toggle_tilemap_visibility.in_set(DebugUpdateSet),
                ),
            );
    }
}

#[derive(Component)]
pub struct MainMap;

#[derive(Component)]
pub struct OverlayMap;

#[derive(Event)]
pub struct LoadLevelEvent(u32);

#[derive(Event, Default)]
pub struct LoadNextLevelEvent;

#[derive(Resource, Default)]
pub struct CurrentLevel(pub u32);

#[derive(Event)]
pub struct TileLevelLoadedEvent {
    pub level_data_map: HashMap<LocationData, TileTypeData>,
}

fn setup(mut load_level_event: EventWriter<LoadLevelEvent>) {
    load_level_event.send(LoadLevelEvent(0));
}

#[allow(clippy::too_many_arguments)]
fn level_data_ready(
    mut commands: Commands,
    tile_map_query: Query<Entity, With<TilemapType>>,
    mut level_data_loaded_event: EventReader<LevelDataLoadedEvent>,
    mut respawn_player: EventWriter<RespawnPlayerEvent>,
    mut tile_level_loaded: EventWriter<TileLevelLoadedEvent>,
    mut despawn_shadows: EventWriter<DespawnShadowsEvent>,
    debug_state: Res<State<DebugState>>,
    level_data_assets: Res<Assets<LevelData>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(level_data) = level_data_loaded_event.read().next() {
        let level_data = level_data_assets.get(level_data.0).unwrap();

        let mut level_data_map = HashMap::new();

        for tile_map_entity in tile_map_query.iter() {
            commands.entity(tile_map_entity).despawn_recursive();
        }

        let tile_map_size = TilemapSize::new(TILE_MAP_SIZE, TILE_MAP_SIZE);

        let tile_set_handle: Handle<Image> = asset_server.load("image/tile/texture_map.png");
        let mut tile_storage = TileStorage::empty(tile_map_size);
        let tile_map_entity = commands.spawn_empty().id();

        let mut overlay_tile_storage = TileStorage::empty(tile_map_size);
        let overlay_tile_map_entity = commands.spawn_empty().id();

        for tile_data in &level_data.tiles {
            let tile_pos = TilePos::new(tile_data.off.x, tile_data.off.y);

            level_data_map.insert(tile_data.off, tile_data.tile_type);

            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tile_map_entity),
                        texture_index: TileTextureIndex(tile_data.tile_type.texture_index()),
                        ..default()
                    },
                    TransformBundle::from_transform(Transform::from_translation(
                        tile_pos_to_world_pos(tile_pos.into(), 0.),
                    )),
                    InheritedVisibility::default(),
                ))
                .id();

            if let Some(over) = &tile_data.over {
                let overlay_tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(overlay_tile_map_entity),
                        texture_index: TileTextureIndex(over.texture_index()),
                        ..default()
                    })
                    .id();

                overlay_tile_storage.set(&tile_pos, overlay_tile_entity);
            }

            let collider_entity = commands
                .spawn((
                    TransformBundle::default(),
                    Collider::cuboid(TILE_SIZE / 2., TILE_SIZE / 2.),
                    InheritedVisibility::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                format!("{}, {}", tile_pos.x, tile_pos.y),
                                default(),
                            ),
                            transform: Transform {
                                translation: Vec3::new(0., 0., 1.),
                                scale: Vec3::splat(0.2),
                                ..default()
                            },
                            visibility: match debug_state.get() {
                                DebugState::On => Visibility::Visible,
                                DebugState::Off => Visibility::Hidden,
                            },
                            ..default()
                        },
                        DebugVisibility,
                    ));
                })
                .id();

            commands.entity(tile_entity).add_child(collider_entity);

            commands.entity(tile_map_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }

        let tile_size = TilemapTileSize::new(TILE_SIZE, TILE_SIZE);
        let grid_size = tile_size.into();

        commands.entity(tile_map_entity).insert((
            TilemapBundle {
                grid_size,
                size: tile_map_size,
                storage: tile_storage,
                tile_size,
                texture: TilemapTexture::Single(tile_set_handle.clone()),
                ..default()
            },
            MainMap,
        ));

        commands.entity(overlay_tile_map_entity).insert((
            TilemapBundle {
                grid_size,
                size: tile_map_size,
                storage: overlay_tile_storage,
                tile_size,
                texture: TilemapTexture::Single(tile_set_handle),
                transform: Transform::from_xyz(0., 0., TILE_MAP_Z_INDEX),
                ..default()
            },
            OverlayMap,
        ));

        respawn_player.send_default();
        tile_level_loaded.send(TileLevelLoadedEvent { level_data_map });
        despawn_shadows.send_default();
    }
}

fn receive_load_level(
    mut load_level_event: EventReader<LoadLevelEvent>,
    mut current_level: ResMut<CurrentLevel>,
) {
    if let Some(load_level) = load_level_event.read().next() {
        println!("Loading level: {}", load_level.0);
        current_level.0 = load_level.0;
    }
}

fn receive_load_next_level(
    current_level: Res<CurrentLevel>,
    mut load_next_level_event: EventReader<LoadNextLevelEvent>,
    mut load_level_event: EventWriter<LoadLevelEvent>,
) {
    if load_next_level_event.read().next().is_some() {
        if current_level.0 < MAX_LEVEL_INDEX {
            load_level_event.send(LoadLevelEvent(current_level.0 + 1));
        } else {
            println!("WIN!");
        }
    }
}

fn toggle_tilemap_visibility(
    mut tile_map_query: Query<&mut Visibility, With<TilemapType>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        for mut tile_map_visibility in tile_map_query.iter_mut() {
            *tile_map_visibility = match *tile_map_visibility {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                Visibility::Inherited => Visibility::Hidden,
            };

            println!("toggle map visibility: {:?}", tile_map_visibility);
        }
    }
}
