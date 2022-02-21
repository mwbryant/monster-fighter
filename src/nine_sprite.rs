use crate::ascii::spawn_ascii_sprite;
use crate::{AsciiSheet, TILE_SIZE};
use bevy::prelude::*;

//XXX all of this is gross, see if u can use a plugin
pub struct NineSpritePlugin;

impl Plugin for NineSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_nine_sprite)
        //        .add_startup_system(spawn_simple_nine_sprite);
            ;
    }
}

#[derive(Copy, Clone)]
pub struct NineSpriteIndices {
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

#[allow(dead_code)]
fn spawn_simple_nine_sprite(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    indices: Res<NineSpriteIndices>,
) {
    spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        11.0 * TILE_SIZE,
        8.0 * TILE_SIZE,
        Vec3::new(2.0 * TILE_SIZE, 2.0 * TILE_SIZE, 0.0),
    );
}

#[derive(Component)]
struct NineSprite;

pub fn spawn_nine_sprite(
    commands: &mut Commands,
    ascii: AsciiSheet,
    indices: NineSpriteIndices,
    width: f32,
    height: f32,
    center: Vec3,
) -> Entity {
    assert!(width >= 2.0 * TILE_SIZE);
    assert!(height >= 2.0 * TILE_SIZE);

    let color = Color::rgb(0.3, 0.3, 0.9);

    let left = -width / 2.0 + 0.5 * TILE_SIZE;
    let right = width / 2.0 - 0.5 * TILE_SIZE;
    let up = height / 2.0 - 0.5 * TILE_SIZE;
    let down = -height / 2.0 + 0.5 * TILE_SIZE;

    let mut sprites = Vec::new();
    sprites.push(spawn_ascii_sprite(
        commands,
        &ascii,
        indices.upper_left_index,
        color,
        Vec3::new(left, up, 0.0),
        Vec3::splat(1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        &ascii,
        indices.vertical_index,
        color,
        Vec3::new(left, 0.0, 0.0),
        Vec3::new(1.0, height / TILE_SIZE - 2.0, 1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        &ascii,
        indices.lower_left_index,
        color,
        Vec3::new(left, down, 0.0),
        Vec3::splat(1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        &ascii,
        indices.horizontal_index,
        color,
        Vec3::new(0.0, down, 0.0),
        Vec3::new(width / TILE_SIZE - 2.0, 1.0, 1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        &ascii,
        indices.horizontal_index,
        color,
        Vec3::new(0.0, up, 0.0),
        Vec3::new(width / TILE_SIZE - 2.0, 1.0, 1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        &ascii,
        indices.upper_right_index,
        color,
        Vec3::new(right, up, 0.0),
        Vec3::splat(1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        &ascii,
        indices.vertical_index,
        color,
        Vec3::new(right, 0.0, 0.0),
        Vec3::new(1.0, height / TILE_SIZE - 2.0, 1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        &ascii,
        indices.lower_right_index,
        color,
        Vec3::new(right, down, 0.0),
        Vec3::splat(1.0),
    ));

    commands
        .spawn()
        .insert(NineSprite)
        .insert(Name::new("NineSpriteBox"))
        //Needs transforms for parent heirarchy system to work
        .insert(Transform {
            translation: center,
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .push_children(&sprites)
        .id()
}
