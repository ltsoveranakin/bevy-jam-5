use std::fmt;
use std::fmt::{Display, Formatter};

use bevy::asset::Asset;
use bevy::math::UVec2;
use bevy::prelude::TypePath;
use bevy_ecs_tilemap::tiles::TilePos;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct LocationData {
    pub x: u32,
    pub y: u32,
}

impl LocationData {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
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

impl Serialize for LocationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{},{}", self.x, self.y);
        serializer.serialize_str(&s)
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

#[derive(Deserialize, Serialize, Hash, Debug, Copy, Clone)]
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

    pub fn from_texture_index(texture_index: u32) -> Self {
        match texture_index {
            0 => Self::Dirt,
            1 => Self::Stone,
            3 => Self::Water,
            _ => unreachable!(),
        }
    }

    pub fn is_hazard(&self) -> bool {
        match self {
            Self::Water => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub enum OverlayData {
    Grass,
}

impl OverlayData {
    pub fn texture_index(&self) -> u32 {
        match self {
            Self::Grass => 2,
        }
    }

    pub fn from_texture_index(texture_index: u32) -> Self {
        match texture_index {
            2 => Self::Grass,
            _ => unreachable!(),
        }
    }
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct TileData {
    pub tile_type: TileTypeData,
    pub off: LocationData,
    pub over: Option<OverlayData>,
}

#[derive(Deserialize, Serialize, Asset, TypePath, Hash, Debug)]
pub struct LevelData {
    pub spawn_location: LocationData,
    pub tiles: Vec<TileData>,
}
