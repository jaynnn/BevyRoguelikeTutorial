use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod wall;

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
) {
}

