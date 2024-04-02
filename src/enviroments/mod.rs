use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::Collider;

mod wall;

use crate::loading;
use crate::player::Player;
use crate::GameState;
use crate::player;

pub struct EnvironmentsPlugin;

impl Plugin for EnvironmentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .add_systems(OnEnter(GameState::Playing), (
                setup,
                wall::setup.after(setup).after(player::spawn_player),
            ));
    }
}

fn setup(
    mut cmds: Commands,
    textures: Res<loading::TextureAssets>,
    query_player: Query<(&Transform, &Entity), With<Player>>,
    
) {

    for (player_transform, collider) in query_player.iter() {
    }
    
    let map_size = TilemapSize { x: 128, y: 128 };
    let tilemap_entity = cmds.spawn_empty().id();
    let tile_storage = TileStorage::empty(map_size);
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    cmds
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(textures.wall_tile.clone()),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        })
        .insert(wall::Wall);
}

