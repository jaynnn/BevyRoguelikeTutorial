use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

pub mod control;
pub mod audio;

use crate::loading::TextureAssets;
use crate::GameState;
use crate::loading;

#[derive(Component)]
pub struct Player;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<control::PlayerAction>::default())
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, 
                (
                    control::move_player.run_if(in_state(GameState::Playing)),
                    audio::control_flying_sound.run_if(in_state(GameState::Playing))
                )
            );
    }
}

pub fn spawn_player(
    mut commands: Commands, 
    textures: Res<TextureAssets>,
    config: Res<loading::ConfigSeatHandles>,
    seat_config: Res<Assets<loading::data::SeatData>>,
) {
    let player_config = seat_config.get(config.player.clone()).unwrap();
    let player_size = player_config.sprite_size;
    let player_collider_size = player_config.collider_size;
    let player_transfrom = player_config.transform;
    let sprite_anchor = player_config.sprite_anchor;

    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(player_size),
                anchor: bevy::sprite::Anchor::Custom(sprite_anchor),
                ..default()
            },
            texture: textures.player.clone(),
            transform: Transform::from_translation(player_transfrom),
            ..Default::default()
        },
        InputManagerBundle::with_map(control::PlayerAction::default_input_map()),
        Velocity::zero(),
        Player,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(player_collider_size.x, player_collider_size.y),
        Name::new("player"),
    ));
}
