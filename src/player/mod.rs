use crate::components::Movement;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

mod control;
mod audio;

pub use control::*;
use audio::*;

#[derive(Component)]
pub struct Player;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, 
                (
                    move_player.run_if(in_state(GameState::Playing)),
                    set_movement_actions.run_if(in_state(GameState::Playing)),
                    control_flying_sound.after(set_movement_actions).run_if(in_state(GameState::Playing))
                )
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.player.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)).with_scale(Vec3::splat(2.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(InputManagerBundle::with_map(PlayerAction::default_input_map()))
        .insert(Movement::default());
}
