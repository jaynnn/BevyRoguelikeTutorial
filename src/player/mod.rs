use crate::components::Movement;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;

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



pub fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    let transfrom = Transform::from_translation(Vec3::new(0., 0., 1.));
    let sprite_size = 32.;
    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..default()
            },
            texture: textures.player.clone(),
            transform: transfrom,
            ..Default::default()
        },
        Player,
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        Movement::default(),
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(sprite_size / 2., sprite_size / 4.),
    ));
}
