use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{self, Rng};

#[derive(Component)]
pub struct Wall;

pub fn setup(
    mut cmds: Commands,
    mut query: Query<(Entity, &TilemapSize, &mut TileStorage), With<Wall>>
) {
    let (entity, wall_map_size, mut wall_storage) = query.single_mut();
    let mut rng = rand::thread_rng();
    for x in 0..wall_map_size.x {
        for y in 0..wall_map_size.y {
            if rng.gen_bool(0.7) {
                continue;
            }
            let tile_pos = TilePos { x, y };
            let tile_entity = cmds
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(entity),
                    ..Default::default()
                })
                .id();
            wall_storage.set(&tile_pos, tile_entity);
        }
    }
    
}


