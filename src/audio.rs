use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource, InstanceHandle, PlaybackState};

//FIXME bad name
pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_system(temp_volume_control)
            .add_system(set_audio_volume)
            .add_system(check_audio_loading);
    }
}

//TODO is there a better key for the hashmap, it would be nice if plugins could add their own maybe
//     or maybe its best to keep all audio file loading in one place
#[derive(Hash, PartialEq, Eq)]
pub enum Clips {
    Hit,
}

pub struct AudioClip {
    pub loaded: bool,
    pub handle: Handle<AudioSource>,
    instance: Option<InstanceHandle>,
}

pub struct AudioState {
    pub clips: HashMap<Clips, AudioClip>,
    pub main_volume: f32,
}

fn temp_volume_control(mut audio_state: ResMut<AudioState>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Up) {
        audio_state.main_volume += 0.10;
    }
    if keyboard.just_pressed(KeyCode::Down) {
        audio_state.main_volume -= 0.10;
    }
    //Behavior is weird outside of this range
    audio_state.main_volume = audio_state.main_volume.clamp(0.0, 1.0);
}

fn check_audio_loading(mut audio_state: ResMut<AudioState>, asset_server: ResMut<AssetServer>) {
    for (_, mut clip) in &mut audio_state.clips {
        if !clip.loaded && asset_server.get_load_state(&clip.handle) == LoadState::Loaded {
            clip.loaded = true;
        }
    }
}

//FIXME Probably can only be run when changes occur but eh
fn set_audio_volume(audio: Res<Audio>, audio_state: Res<AudioState>) {
    audio.set_volume(audio_state.main_volume);
}

fn load_audio(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let hit_handle = asset_server.load("hit.wav");
    let mut audio_state = AudioState {
        clips: HashMap::default(),
        main_volume: 0.5,
    };
    audio_state.clips.insert(
        Clips::Hit,
        AudioClip {
            loaded: false,
            handle: hit_handle,
            instance: None,
        },
    );

    commands.insert_resource(audio_state);
}

#[allow(dead_code)]
pub fn play_single_sound(audio: Res<Audio>, clip: &mut AudioClip) {
    if !clip.loaded {
        return;
    }
    if clip.instance == None
        || audio.state(clip.instance.clone().unwrap()) == PlaybackState::Stopped
    {
        clip.instance = Some(audio.play(clip.handle.clone()));
    }
}
