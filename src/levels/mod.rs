use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::debug::DebugVisibility;
use crate::levels::data::LevelData;
use crate::levels::level_loader::{LevelDataLoadedEvent, LevelLoaderPlugin};
use crate::player::Player;

mod data;
mod level_loader;

const TILE_MAP_SIZE: u32 = 32;
const TILE_SIZE: f32 = 16.;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelLoaderPlugin)
            .add_event::<LoadLevelEvent>()
            .add_systems(Startup, setup);
    }
}

#[derive(Event)]
pub struct LoadLevelEvent(i32);

fn setup(mut load_level_event: EventWriter<LoadLevelEvent>) {
    load_level_event.send(LoadLevelEvent(0));
}

fn level_data_ready(
    mut commands: Commands,
    tilemap_query: Query<Entity, With<TilemapType>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut level_data_loaded_event: EventReader<LevelDataLoadedEvent>,
    level_data_assets: Res<Assets<LevelData>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(level_data) = level_data_loaded_event.read().next() {
        let mut player_transform = player_query.single_mut();

        let level_data = level_data_assets.get(level_data.0).unwrap();
        if let Ok(tilemap_entity) = tilemap_query.get_single() {
            commands.entity(tilemap_entity).despawn_recursive();
        }

        let tile_map_size = TilemapSize {
            x: TILE_MAP_SIZE,
            y: TILE_MAP_SIZE,
        };

        let tile_set_handle: Handle<Image> = asset_server.load("image/tile/tile_set.png");
        let mut tile_storage = TileStorage::empty(tile_map_size);
        let tile_map_entity = commands.spawn_empty().id();

        for tile_data in &level_data.tiles {
            let tile_pos = TilePos::new(tile_data.off.x, tile_data.off.y);

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tile_map_entity),
                    texture_index: TileTextureIndex(tile_data.tile_type.texture_index()),
                    ..default()
                })
                .id();

            commands
                .spawn((
                    TransformBundle::from_transform(Transform::from_translation(
                        tile_pos_to_world_pos(tile_pos.into(), 0.),
                    )),
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
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        DebugVisibility,
                    ));
                });

            commands.entity(tile_map_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }

        let tile_size = TilemapTileSize::new(TILE_SIZE, TILE_SIZE);
        let grid_size = tile_size.into();

        commands.entity(tile_map_entity).insert(TilemapBundle {
            grid_size,
            size: tile_map_size,
            storage: tile_storage,
            tile_size,
            texture: TilemapTexture::Single(tile_set_handle),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        });

        player_transform.translation = tile_pos_to_world_pos(level_data.spawn_location.into(), 0.);
    }
}

fn tile_pos_to_world_pos(tile_pos: UVec2, z_index: f32) -> Vec3 {
    (tile_pos * TILE_SIZE as u32).as_vec2().extend(z_index)
}
