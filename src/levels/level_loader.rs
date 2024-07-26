use std::fmt::Formatter;
use std::io;

use bevy::asset::{AssetLoader, AsyncReadExt, io::Reader, LoadContext};
use bevy::prelude::*;
use bevy::utils::HashSet;
use thiserror::Error;

use crate::levels::{level_data_ready, LoadLevelEvent};
use crate::levels::data::{LevelData, LocationData};

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelDataLoadedEvent>()
            .init_resource::<LevelDataHandleRes>()
            .init_asset_loader::<LevelJSONAssetLoader>()
            .init_asset::<LevelData>()
            .add_systems(Update, (load_level, set_loaded_level, level_data_ready));
    }
}

#[derive(Default)]
struct LevelJSONAssetLoader;

#[derive(Debug)]
enum InvalidLevelErrorReason {
    DuplicateTileLocation(LocationData),
}

#[derive(Debug, Error)]
struct InvalidLevelError {
    reason: InvalidLevelErrorReason,
}

impl InvalidLevelError {
    fn new(reason: InvalidLevelErrorReason) -> Self {
        Self { reason }
    }
}

impl std::fmt::Display for InvalidLevelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.reason {
            InvalidLevelErrorReason::DuplicateTileLocation(loc) => {
                write!(f, "Duplicate tile location at: {}", loc)
            }
        }
    }
}

#[derive(Debug, Error)]
enum LevelJSONAssetLoaderError {
    #[error("Could not parse json: {0}")]
    SerdeParse(#[from] serde_json::Error),
    #[error("Could not read file: {0}")]
    IO(#[from] io::Error),
    #[error("Level data was invalid: {0}")]
    InvalidLevel(#[from] InvalidLevelError),
}

impl AssetLoader for LevelJSONAssetLoader {
    type Asset = LevelData;
    type Settings = ();
    type Error = LevelJSONAssetLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<LevelData, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let level_json: LevelData = serde_json::from_slice(bytes.as_slice())?;

        let mut tiles_set = HashSet::new();

        for tile in level_json.tiles.iter() {
            if tiles_set.contains(&tile.off) {
                return Err(InvalidLevelError::new(
                    InvalidLevelErrorReason::DuplicateTileLocation(tile.off),
                )
                .into());
            } else {
                tiles_set.insert(tile.off);
            }
        }

        Ok(level_json)
    }

    fn extensions(&self) -> &[&str] {
        &["lvl"]
    }
}

#[derive(Resource, Default)]
pub struct LevelDataHandleRes(pub Option<Handle<LevelData>>);

fn load_level(
    mut set_level_event: EventReader<LoadLevelEvent>,
    asset_server: Res<AssetServer>,
    mut level_data_json_handle: ResMut<LevelDataHandleRes>,
) {
    if let Some(level_id) = set_level_event.read().next() {
        let level_handle = asset_server.load(format!("level/level{}.lvl", level_id.0));
        level_data_json_handle.0 = Some(level_handle);
    }
}

#[derive(Event)]
pub struct LevelDataLoadedEvent(pub AssetId<LevelData>);

fn set_loaded_level(
    mut asset_event_reader: EventReader<AssetEvent<LevelData>>,
    mut level_loaded_event: EventWriter<LevelDataLoadedEvent>,
) {
    for asset_event in asset_event_reader.read() {
        if let AssetEvent::LoadedWithDependencies { id } = asset_event {
            level_loaded_event.send(LevelDataLoadedEvent(*id));
        }
    }
}
