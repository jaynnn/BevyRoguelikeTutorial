use bevy::{ecs::query, prelude::*, tasks::futures_lite::stream::StepBy};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::loading;
use crate::helper;

#[derive(Event)]
pub struct EventCreateRoom {
    // origin is the center of the room
    pub origin: Vec2,
    pub half_size: Vec2,
    pub tilemap_entity: Entity,
}

#[derive(Event)]
pub struct EventCreateWall {
    pub st: Vec2,
    pub ed: Vec2,
    pub tilemap_entity: Entity,
}

#[derive(Event)]
pub struct EventCreateTiles {
    pub tiles: Vec<Vec2>,
    pub tilemap_entity: Entity,
}

#[derive(Component)]
pub struct WallTiles;

#[derive(Component)]
pub struct Wall;

pub fn setup(
    mut cmds: Commands,
    textures: Res<loading::TextureAssets>,
    config: Res<loading::ConfigTileHandles>,
    tile_config: Res<Assets<loading::data::TileData>>,
    mut create_room_writer: EventWriter<EventCreateRoom>,
) {
    let wall_config = tile_config.get(config.wall.clone()).unwrap();
    let map_size = wall_config.map_size;
    let tile_size_config = wall_config.tile_size;

    // create
    let map_size = TilemapSize::from(map_size);
    let tilemap_entity = cmds.spawn_empty().id();
    let tile_storage = TileStorage::empty(map_size);
    let tile_size = TilemapTileSize::from(tile_size_config);
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    cmds
        .entity(tilemap_entity)
        .insert((TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(textures.wall_tile.clone()),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        }, WallTiles));

    create_room_writer.send(EventCreateRoom {
        origin: Vec2::new(0.0, 0.0),
        half_size: Vec2::new(10.0 * tile_size.x, 10.0 * tile_size.y),
        tilemap_entity,
    });
    create_room_writer.send(EventCreateRoom {
        origin: Vec2::new(100.0, 100.0),
        half_size: Vec2::new(10.0 * tile_size.x, 10.0 * tile_size.y),
        tilemap_entity,
    });
}

pub fn create_room(
    mut query: Query<&TilemapTileSize, With<WallTiles>>,
    mut event_create_room: EventReader<EventCreateRoom>,
    mut event_create_tile: EventWriter<EventCreateTiles>,
) {
    for room in event_create_room.read() {
        let mut tiles = vec![];
        if let Ok(tile_size) = query.get_mut(room.tilemap_entity) {
            let st = room.origin - room.half_size;
            let ed = st + (room.half_size * 2.0);
            for x in helper::FloatStep::new(st.x, ed.x, tile_size.x) {
                for y in helper::FloatStep::new(st.y, ed.y, tile_size.y) {
                    if x == st.x || x == ed.x - tile_size.x || y == st.y || y == ed.y - tile_size.y {
                        let transform = Vec2::new(x, y);
                        tiles.push(transform);
                    }
                }
            }
        }
        println!("{}", tiles.len());
        event_create_tile.send(EventCreateTiles {
            tiles,
            tilemap_entity: room.tilemap_entity,
        });
    };
}

pub fn create_tile(
    mut cmds: Commands,
    mut query: Query<(&mut TileStorage, &TilemapSize, &Transform, &TilemapGridSize, &TilemapTileSize, &TilemapType), With<WallTiles>>,
    mut event_create_tile: EventReader<EventCreateTiles>,
) {
    for tile in event_create_tile.read() {
        let (mut storage, tilemap_size, map_transform, grid_size, tile_size, tile_type) = query.get_mut(tile.tilemap_entity).unwrap();
        let half_tile_size_x = tile_size.x / 2.0;
        let half_tile_size_y = tile_size.y / 2.0;
        for transform in tile.tiles.iter() {
            let tile_transform: Vec2 = {
                let pos = Vec4::from((*transform, 0.0, 1.0));
                let tile_transform = map_transform.compute_matrix().inverse() * pos;
                tile_transform.xy()
            };
            let tile_pos = TilePos::from_world_pos(&tile_transform, tilemap_size, grid_size, tile_type).unwrap();
            let tile_entity = cmds
                .spawn((TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tile.tilemap_entity),
                    ..Default::default()
                },
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(map_transform.translation.x + tile_pos.x as f32 * tile_size.x, map_transform.translation.y + tile_pos.y as f32 * tile_size.y, 0.0)),
                    ..default()
                },
                Wall,
                RigidBody::Fixed,
                Collider::cuboid(half_tile_size_x, half_tile_size_y),
            ))
            .id();
            storage.set(&tile_pos, tile_entity);
        }
    }
}



