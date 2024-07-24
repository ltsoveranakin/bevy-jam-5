use bevy::asset::{AssetLoader, AsyncReadExt, io::Reader, LoadContext};
use bevy::prelude::*;

use crate::levels::{level_data_ready, LoadLevelEvent};
use crate::levels::data::LevelData;

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

impl AssetLoader for LevelJSONAssetLoader {
    type Asset = LevelData;
    type Settings = ();
    type Error = std::io::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<LevelData, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let level_json: LevelData = serde_json::from_slice(bytes.as_slice())?;
        Ok(level_json)
    }

    fn extensions(&self) -> &[&str] {
        &["lvl"]
    }
}

#[derive(Resource, Default)]
struct LevelDataHandleRes(Option<Handle<LevelData>>);

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
