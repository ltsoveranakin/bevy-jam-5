use bevy::prelude::*;

use crate::levels::{HALF_TILE_SIZE, TILE_SIZE};

pub fn tile_pos_to_world_pos(tile_pos: UVec2, z_index: f32) -> Vec3 {
    tile_pos_to_world_pos_2d(tile_pos).extend(z_index)
}

pub fn tile_pos_to_world_pos_2d(tile_pos: UVec2) -> Vec2 {
    (tile_pos * TILE_SIZE as u32).as_vec2()
}

pub fn world_pos_to_tile_pos(world_pos: Vec3) -> UVec2 {
    ((world_pos + HALF_TILE_SIZE) / TILE_SIZE)
        .floor()
        .as_uvec3()
        .truncate()
}
