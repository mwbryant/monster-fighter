use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
//TODO how to do #define better or better yet remove on --release
pub const ENABLE_INSPECTOR: bool = true;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());
        if ENABLE_INSPECTOR {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}
