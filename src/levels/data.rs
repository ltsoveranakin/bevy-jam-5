use std::fmt;
use std::fmt::{Display, Formatter};

use bevy::asset::Asset;
use bevy::math::UVec2;
use bevy::prelude::TypePath;
use bevy_ecs_tilemap::tiles::TilePos;
use serde::{Deserialize, Deserializer};
use serde::de;
use serde::de::Visitor;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct LocationData {
    pub x: u32,
    pub y: u32,
}

struct LocationVisitor;

impl<'de> Visitor<'de> for LocationVisitor {
    type Value = LocationData;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string with two comma separated integers with the format x,y")
    }

    fn visit_str<E>(self, value: &str) -> Result<LocationData, E>
    where
        E: de::Error,
    {
        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() != 2 {
            return Err(de::Error::invalid_value(de::Unexpected::Str(value), &self));
        }
        let x = parts[0].parse::<u32>().map_err(de::Error::custom)?;
        let y = parts[1].parse::<u32>().map_err(de::Error::custom)?;
        Ok(LocationData { x, y })
    }
}

impl<'de> Deserialize<'de> for LocationData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(LocationVisitor)
    }
}

impl Display for LocationData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
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
    Water,
}

impl TileTypeData {
    pub fn texture_index(&self) -> u32 {
        match self {
            Self::Dirt => 0,
            Self::Stone => 1,
            Self::Water => 3,
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum OverlayData {
    Grass,
}

impl OverlayData {
    pub fn texture_index(&self) -> u32 {
        match self {
            Self::Grass => 2,
        }
    }
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
