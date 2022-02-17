use crate::ascii::spawn_ascii_sprite;
use crate::{AsciiSheet, TILE_SIZE};
use bevy::prelude::*;

//XXX all of this is gross, see if u can use a plugin
pub struct NineSpritePlugin;

impl Plugin for NineSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_nine_sprite)
            .add_startup_system(spawn_nine_sprite);
    }
}

struct NineSpriteIndices {
    upper_left_index: usize,
    upper_right_index: usize,
    lower_left_index: usize,
    lower_right_index: usize,
    horizontal_index: usize,
    vertical_index: usize,
}

fn setup_nine_sprite(mut commands: Commands) {
    //Indices on ascii sheet
    commands.insert_resource(NineSpriteIndices {
        upper_left_index: 13 * 16 + 10,
        upper_right_index: 11 * 16 + 15,
        lower_left_index: 12 * 16,
        lower_right_index: 13 * 16 + 9,
        horizontal_index: 12 * 16 + 4,
        vertical_index: 11 * 16 + 3,
    });
}

fn spawn_nine_sprite(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    indices: Res<NineSpriteIndices>,
) {
    let color = Color::rgb(0.3, 0.3, 0.9);
    spawn_ascii_sprite(
        &mut commands,
        &ascii,
        indices.upper_left_index,
        color,
        Vec3::new(1.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0),
    );
    spawn_ascii_sprite(
        &mut commands,
        &ascii,
        indices.vertical_index,
        color,
        Vec3::new(1.0 * TILE_SIZE, -3.0 * TILE_SIZE, 0.0),
    );
    spawn_ascii_sprite(
        &mut commands,
        &ascii,
        indices.lower_left_index,
        color,
        Vec3::new(1.0 * TILE_SIZE, -4.0 * TILE_SIZE, 0.0),
    );
    spawn_ascii_sprite(
        &mut commands,
        &ascii,
        indices.horizontal_index,
        color,
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0),
    );
    spawn_ascii_sprite(
        &mut commands,
        &ascii,
        indices.horizontal_index,
        color,
        Vec3::new(2.0 * TILE_SIZE, -4.0 * TILE_SIZE, 0.0),
    );
    spawn_ascii_sprite(
        &mut commands,
        &ascii,
        indices.upper_right_index,
        color,
        Vec3::new(3.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0),
    );
    spawn_ascii_sprite(
        &mut commands,
        &ascii,
        indices.vertical_index,
        color,
        Vec3::new(3.0 * TILE_SIZE, -3.0 * TILE_SIZE, 0.0),
    );
    spawn_ascii_sprite(
        &mut commands,
        &ascii,
        indices.lower_right_index,
        color,
        Vec3::new(3.0 * TILE_SIZE, -4.0 * TILE_SIZE, 0.0),
    );
}
