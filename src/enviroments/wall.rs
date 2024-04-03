use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::loading;

#[derive(Component)]
pub struct Wall;

pub fn setup(
    mut cmds: Commands,
    textures: Res<loading::TextureAssets>,
    config: Res<loading::ConfigTileHandles>,
    tile_config: Res<Assets<loading::data::TileData>>,
) {
    let wall_config = tile_config.get(config.wall.clone()).unwrap();
    let map_size = wall_config.map_size;
    let tile_size = wall_config.tile_size;
    let collider_size = wall_config.collider_size;

    // create
    let map_size = TilemapSize::from(map_size);
    let tilemap_entity = cmds.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    let tile_size = TilemapTileSize::from(tile_size);
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    create_room(&mut cmds, &mut tile_storage, tilemap_entity, wall_config, Vec2::new(64.0, 64.0), Vec2::new(10.0, 10.0));
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
        });
}

fn create_room(cmds: &mut Commands, storage: &mut TileStorage, e: Entity, wall_config: &loading::data::TileData, pos: Vec2, size: Vec2) {
    let map_size = TilemapSize::from(wall_config.map_size);
    let tile_size = TilemapTileSize::from(wall_config.tile_size);
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    let collider_size = wall_config.collider_size;

    let st_x = (pos.x - size.x / 2.0) as i32;
    let ed_x = (pos.x + size.x / 2.0) as i32;
    let st_y = (pos.y - size.y / 2.0) as i32;
    let ed_y = (pos.y + size.y / 2.0) as i32;
    
    for x in st_x..ed_x {
        for y in st_y..ed_y {
            if x == st_x || x == ed_x - 1 || y == st_y || y == ed_y - 1 {
                if let Some(tile_pos) = TilePos::from_i32_pair(x, y, &map_size) {
                    let translation2 = tile_pos.center_in_world(&grid_size, &map_type);
                    let transform = Transform::from_translation(Vec3::new(translation2.x,translation2.y, 1.0));
                    println!("{:?}", transform);
                    let tile_entity = cmds
                        .spawn((
                            TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(e),
                                ..Default::default()
                            },
                            Wall,
                            SpriteBundle {
                                transform,
                                ..default()
                            },
                            RigidBody::Fixed,
                            Collider::cuboid(collider_size.x, collider_size.y),
                        ))
                        .id();
                    storage.set(&tile_pos, tile_entity);
                }
            }
        }
    }
}


