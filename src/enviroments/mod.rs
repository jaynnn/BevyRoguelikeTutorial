use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod wall;
pub mod tile;

use crate::GameState;
use crate::player;

pub struct EnvironmentsPlugin;

impl Plugin for EnvironmentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .add_event::<wall::EventCreateRoom>()
            .add_event::<wall::EventCreateTiles>()
            .add_systems(OnEnter(GameState::Playing), (
                setup,
                wall::setup.after(setup).after(player::spawn_player),
                wall::create_room.after(wall::setup),
                wall::create_tile.after(wall::create_room)
            ));
    }
}

fn setup(
) {
}

