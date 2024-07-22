use bevy::asset::Asset;
use bevy::prelude::TypePath;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LocationData {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize, Debug)]
pub enum TileTypeData {
    Dirt,
}

#[derive(Deserialize, Debug)]
pub enum OverlayData {
    Grass,
}

#[derive(Deserialize, Debug)]
pub struct TileData {
    pub tile_type: TileTypeData,
    pub off: LocationData,
    pub over: Option<OverlayData>,
}

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct LevelData {
    pub spawn_location: LocationData,
    pub tiles: Vec<TileData>,
}
