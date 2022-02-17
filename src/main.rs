//I personally like the consistency of "field: value" more than removing the copy
#![allow(clippy::redundant_field_names)]

#[allow(unused_imports)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowMode;
use bevy_inspector_egui::WorldInspectorPlugin;

mod ascii;
mod combat;
mod nine_sprite;
mod player;
mod tilemap;

use ascii::{AsciiPlugin, AsciiSheet};
use combat::CombatPlugin;
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
        .add_plugin(WorldInspectorPlugin::new())
        .add_state(GameState::Overworld)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TileMapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(NineSpritePlugin)
        .add_plugin(AsciiPlugin)
        .add_startup_system(spawn_camera)
        .run();
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
