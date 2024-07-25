use bevy::prelude::*;

use crate::levels::TILE_SIZE;

pub fn tile_pos_to_world_pos(tile_pos: UVec2, z_index: f32) -> Vec3 {
    (tile_pos * TILE_SIZE as u32).as_vec2().extend(z_index)
}
