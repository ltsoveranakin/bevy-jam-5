use bevy::asset::Asset;
use bevy::math::UVec2;
use bevy::prelude::TypePath;
use bevy_ecs_tilemap::tiles::TilePos;
use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct LocationData {
    pub x: u32,
    pub y: u32,
}

impl From<LocationData> for TilePos {
    fn from(value: LocationData) -> Self {
        TilePos::new(value.x, value.y)
    }
}

impl From<LocationData> for UVec2 {
    fn from(value: LocationData) -> Self {
        UVec2::new(value.x, value.y)
    }
}

#[derive(Deserialize, Debug)]
pub enum TileTypeData {
    Dirt,
    Stone,
}

impl TileTypeData {
    pub fn texture_index(&self) -> u32 {
        match self {
            Self::Dirt => 0,
            Self::Stone => 1,
        }
    }
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
