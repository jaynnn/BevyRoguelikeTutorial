use bevy::prelude::*;
use serde::Deserialize;


#[derive(Debug, Asset, Deserialize, TypePath)]
pub struct SeatData {
    pub sprite_size: Vec2,
    pub transform: Vec3,
    pub collider_size: Vec2,
    pub collider_offset: Vec3,
}

#[derive(Debug, Asset, Deserialize, TypePath)]
pub struct TileData {
    pub tile_size: Vec2,
    pub map_size: UVec2,
    pub collider_size: Vec2,
}
