use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::*;
use bevy::utils::ConditionalSendFuture;
use serde::{Deserialize, Serialize};

use crate::levels::LoadLevelEvent;

struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetLoadingAssetEvent>()
            .init_resource::<LevelDataHandleRes>()
            .init_asset_loader::<LevelJSONAssetLoader>()
            .init_asset::<LevelData>();
    }
}

#[derive(Default)]
struct LevelJSONAssetLoader;

impl AssetLoader for LevelJSONAssetLoader {
    type Asset = LevelData;
    type Settings = ();
    type Error = ();

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let level_json: LevelData = serde_json::from_slice(bytes.as_slice())?;
        Ok(level_json)
    }

    fn extensions(&self) -> &[&str] {
        &["lvl"]
    }
}

struct LocationJSON {
    x: u32,
    y: u32,
}

enum TileTypeJSON {
    Dirt,
}

struct TileJSON {
    tile_type: TileTypeJSON,
}

#[derive(Serialize, Deserialize, Asset)]
struct LevelData {
    spawn_location: LocationJSON,
    tiles: Vec<TileJSON>,
}

#[derive(Resource, Default)]
struct LevelDataHandleRes(Option<Handle<LevelData>>);

fn load_level(
    mut set_level: EventReader<LoadLevelEvent>,
    asset_server: AssetServer,
    mut level_data_json_handle: ResMut<LevelDataHandleRes>,
) {
    if let Some(level_id) = set_level.read().next() {
        let level_handle = asset_server.load(format!("level{}.json", level_id.0));
        level_data_json_handle.0 = Some(level_handle);
    }
}

#[derive(Event)]
struct SetLoadingAssetEvent(AssetId<LevelData>);

fn set_loaded_level(
    mut asset_event_reader: EventReader<AssetEvent<LevelData>>,
    mut set_loading_asset_event: EventWriter<SetLoadingAssetEvent>,
) {
    for asset_event in asset_event_reader.read() {
        match asset_event {
            AssetEvent::Added { id } => {
                set_loading_asset_event.send(SetLoadingAssetEvent(*id));
            }
            AssetEvent::Modified { id } => {
                set_loading_asset_event.send(SetLoadingAssetEvent(*id));
            }
            AssetEvent::Removed { .. } => {}
            AssetEvent::Unused { .. } => {}
            AssetEvent::LoadedWithDependencies { .. } => {}
        }
    }
}
