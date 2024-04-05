use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub enum TileType {
    Floor,
    Wall,
    Door,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub transform: Vec2,
}