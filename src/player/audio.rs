use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::audio::FlyingAudio;

pub fn control_flying_sound(
    player: Query<&Velocity, (Changed<Velocity>, With<Player>)>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        for vel in player.iter() {
            match instance.state() {
                PlaybackState::Paused { .. } => {
                    if vel.linvel.length() > 0.0 {
                        instance.resume(AudioTween::default());
                    }
                }
                PlaybackState::Playing { .. } => {
                    if vel.linvel.length() == 0.0 {
                        instance.pause(AudioTween::default());
                    }
                }
                _ => {}
            }
        }
    }
}
