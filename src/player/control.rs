
use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

pub const FOLLOW_EPSILON: f32 = 5.;

use crate::{components::Movement, player::Player};

#[derive(Actionlike, Debug, Clone, Eq, PartialEq, Reflect, Hash)]
pub enum PlayerAction {
    Move,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Move, DualAxis::left_stick());

        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map
    }
}


pub fn set_movement_actions(
    touch_input: Res<Touches>,
    mut player: Query<(&mut Movement, &Transform, &ActionState<PlayerAction>), With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    for (mut movement, player, action_state) in player.iter_mut() {
        if action_state.pressed(&PlayerAction::Move) {
            let mut player_movement = action_state
                        .clamped_axis_pair(&PlayerAction::Move)
                        .unwrap()
                        .xy();
            if let Some(touch_position) = touch_input.first_pressed_position() {
                let (camera, camera_transform) = camera.single();
                if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
                {
                    let diff = touch_position - player.translation.xy();
                    if diff.length() > FOLLOW_EPSILON {
                        player_movement = diff.normalize();
                    }
                }
            }
            movement.dir = Some(player_movement.normalize());
        } else {
            movement.dir = None;
        }
    }
}

pub fn move_player(
    time: Res<Time>,
    mut player_query: Query<(&Movement, &mut Transform), With<Player>>,
) {
    for (movement, mut player_transform) in player_query.iter_mut() {
        if movement.dir.is_none() {
            continue;
        }
        let speed = 150.;
        let movement = Vec3::new(
            movement.dir.unwrap().x * speed * time.delta_seconds(),
            movement.dir.unwrap().y * speed * time.delta_seconds(),
            0.,
        );
        player_transform.translation += movement;
    }
}
