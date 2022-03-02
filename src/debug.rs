use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

#[cfg(debug_assertions)]
pub const ENABLE_INSPECTOR: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_INSPECTOR: bool = false;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default());
        }
        if ENABLE_INSPECTOR {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}
