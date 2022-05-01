use bevy::prelude::*;

use crate::player::{JumpEvent, OffscreenDeathEvent};
use crate::level::VictoryEvent;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct GameAudioPlugin;

struct AudioChannels {
    background: AudioChannel,
    sfx: AudioChannel,
}

impl FromWorld for AudioChannels {
    fn from_world(world: &mut World) -> Self {
        // You have full access to anything in the ECS from here.
        // For instance, you can mutate other resources:
        let background = AudioChannel::new("background".to_owned());
        let sfx = AudioChannel::new("sfx".to_owned());
        AudioChannels { background, sfx }
    }
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioChannels>()
            .add_startup_system(start_background_audio)
            .add_system(jump_audio)
            .add_system(fall_audio)
            .add_system(victory_audio)
            .add_plugin(AudioPlugin);
        //   .add_system(handle_input)
    }
}

fn start_background_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
) {
    audio.play_looped_in_channel(
        asset_server.load("sounds/level1_bg.wav"),
        &channels.background,
    );
    audio.set_volume_in_channel(0.5, &channels.background)
}

fn jump_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
    mut ev_jump: EventReader<JumpEvent>,
) {
    for ev in ev_jump.iter() {
        let jump_number = ev.0 + 1;
        eprintln!("Jump Event {:?}", jump_number);
        match jump_number {
            1 => {
                audio.play_in_channel(asset_server.load("sounds/jump1.wav"), &channels.sfx);
            }
            2 => {
                audio.play_in_channel(asset_server.load("sounds/jump2.wav"), &channels.sfx);
            }
            3 => {
                audio.play_in_channel(asset_server.load("sounds/jump3.wav"), &channels.sfx);
            }
            _ => (),
        }
    }
}

fn fall_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
    mut event_reader: EventReader<OffscreenDeathEvent>,
) {
    for ev in event_reader.iter() {
        audio.play_in_channel(asset_server.load("sounds/falling_noise.wav"), &channels.sfx);
    }
}

fn victory_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
    mut event_reader: EventReader<VictoryEvent>,
) {
    for ev in event_reader.iter() {
        audio.play_in_channel(asset_server.load("sounds/victory_sound.wav"), &channels.sfx);
        audio.stop_channel(&channels.background);
        audio.play_looped_in_channel(asset_server.load("sounds/intro.wav"), &channels.background);

    }
}