//I personally like the consistency of "field: value" more than removing the copy
#![allow(clippy::redundant_field_names)]

#[allow(unused_imports)]
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::{asset::LoadState, render::camera::ScalingMode};
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource, InstanceHandle, PlaybackState};

mod ascii;
mod ascii_text;
mod combat;
mod debug;
mod enemy;
mod nine_sprite;
mod player;
mod screen_fadeout;
mod tilemap;

use ascii::{spawn_ascii_sprite, AsciiPlugin, AsciiSheet};
use combat::CombatPlugin;
use debug::DebugPlugin;
use nine_sprite::NineSpritePlugin;
use player::PlayerPlugin;
use tilemap::TileMapPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.10;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    //Menu,
    Overworld,
    Combat,
}

fn main() {
    let height = 900.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Monster Fighter".to_string(),
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_startup_system(prepare_audio)
        .add_system(check_audio_loading)
        //.add_system(play_single_sound)
        .add_state(GameState::Overworld)
        .add_plugin(DebugPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(NineSpritePlugin)
        .add_plugin(AsciiPlugin)
        .add_startup_system(spawn_camera)
        //.add_startup_system(spawn_dummy_sprite)
        .add_system(frame_limiter)
        .run();
}

struct AudioState {
    audio_loaded: bool,
    hit_handle: Handle<AudioSource>,
    hit_instance: Option<InstanceHandle>,
}

fn check_audio_loading(mut audio_state: ResMut<AudioState>, asset_server: ResMut<AssetServer>) {
    if audio_state.audio_loaded
        || LoadState::Loaded != asset_server.get_load_state(&audio_state.hit_handle)
    {
        return;
    }
    audio_state.audio_loaded = true;
}

fn prepare_audio(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let hit_handle = asset_server.load("hit.wav");
    let audio_state = AudioState {
        audio_loaded: false,
        hit_handle: hit_handle,
        hit_instance: None,
    };
    commands.insert_resource(audio_state);
}

#[allow(dead_code)]
fn play_single_sound(audio: Res<Audio>, mut audio_state: ResMut<AudioState>) {
    if !audio_state.audio_loaded {
        return;
    }
    if audio_state.hit_instance == None
        || audio.state(audio_state.hit_instance.clone().unwrap()) == PlaybackState::Stopped
    {
        audio_state.hit_instance = Some(audio.play(audio_state.hit_handle.clone()));
    }
}

#[allow(dead_code)]
fn spawn_dummy_sprite(mut commands: Commands, ascii: Res<AsciiSheet>) {
    spawn_ascii_sprite(
        &mut commands,
        &*ascii,
        1,
        Color::RED,
        Vec3::default(),
        Vec3::splat(1.0),
    );
}

// Janky but keeps my laptop from hitting 400fps and using 100% cpu
// https://github.com/bevyengine/bevy/issues/1343
fn frame_limiter() {
    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(10));
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    commands.spawn_bundle(camera);
}
