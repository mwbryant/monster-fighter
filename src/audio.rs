use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource, InstanceHandle, PlaybackState};

//FIXME bad name
pub struct MyAudioPlugin;

impl Plugin for MyAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_system(check_audio_loading);
    }
}

pub struct AudioClip {
    pub handle: Handle<AudioSource>,
    instance: Option<InstanceHandle>,
}

pub struct AudioState {
    pub audio_loaded: bool,
    pub hit_clip: AudioClip,
}

fn check_audio_loading(mut audio_state: ResMut<AudioState>, asset_server: ResMut<AssetServer>) {
    if audio_state.audio_loaded
        || LoadState::Loaded != asset_server.get_load_state(&audio_state.hit_clip.handle)
    {
        return;
    }
    audio_state.audio_loaded = true;
}

fn load_audio(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let hit_handle = asset_server.load("hit.wav");
    let audio_state = AudioState {
        audio_loaded: false,
        hit_clip: AudioClip {
            handle: hit_handle,
            instance: None,
        },
    };
    commands.insert_resource(audio_state);
}

#[allow(dead_code)]
pub fn play_single_sound(audio: Res<Audio>, clip: &mut AudioClip) {
    //TODO it would be nice to check for loaded audio...
    //if !audio_state.audio_loaded {
    //return;
    //}
    if clip.instance == None
        || audio.state(clip.instance.clone().unwrap()) == PlaybackState::Stopped
    {
        clip.instance = Some(audio.play(clip.handle.clone()));
    }
}
