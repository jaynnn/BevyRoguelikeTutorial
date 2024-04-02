use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use bevy_kira_audio::prelude::*;

use crate::components::Movement;
use crate::player::Player;
use crate::audio::FlyingAudio;

pub fn control_flying_sound(
    player: Query<&Movement, With<Player>>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        for player_movement in player.iter() {
            match instance.state() {
                PlaybackState::Paused { .. } => {
                    if player_movement.dir.is_some() {
                        instance.resume(AudioTween::default());
                    }
                }
                PlaybackState::Playing { .. } => {
                    if player_movement.dir.is_none() {
                        instance.pause(AudioTween::default());
                    }
                }
                _ => {}
            }
        }
    }
}
