
use leafwing_input_manager::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const FOLLOW_EPSILON: f32 = 5.;

use crate::player::Player;

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

pub fn move_player(
    touch_input: Res<Touches>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut player_info: Query<(&Transform, &mut Velocity, &ActionState<PlayerAction>), With<Player>>,
) {
    for (player_transform, mut rb_vels, action_state) in player_info.iter_mut() {
        if action_state.pressed(&PlayerAction::Move) {
            let mut player_movement = action_state
                        .clamped_axis_pair(&PlayerAction::Move)
                        .unwrap()
                        .xy();
            if let Some(touch_position) = touch_input.first_pressed_position() {
                let (camera, camera_transform) = camera.single();
                if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
                {
                    let diff = touch_position - player_transform.translation.xy();
                    if diff.length() > FOLLOW_EPSILON {
                        player_movement = diff.normalize();
                    }
                }
            }
    
            let move_delta = player_movement.normalize();
    
            let speed = 150.;
            if move_delta.length() > 0. {
                // println!("+++++++ {:?}", move_delta.length());
                rb_vels.linvel = move_delta * speed;
                // println!("+++++++222222222 {:?}", rb_vels.linvel.length());
            }
        } else {
            if rb_vels.linvel.length() > 0. {
                // println!("========{:?}", rb_vels.linvel.length());
                rb_vels.linvel = Vec2::ZERO.into();
            }
        }
    }
}
