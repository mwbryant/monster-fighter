use crate::{spawn_ascii_sprite, AsciiSheet, TILE_SIZE};
use bevy::prelude::*;

pub fn spawn_ascii_text(
    commands: &mut Commands,
    ascii: AsciiSheet,
    to_print: &str,
    left_center: Vec3,
) -> Entity {
    let color = Color::rgb(0.8, 0.8, 0.8);

    let mut sprites = Vec::new();
    for (i, char) in to_print.chars().enumerate() {
        sprites.push(spawn_ascii_sprite(
            commands,
            &ascii,
            char as usize,
            color,
            Vec3::new(i as f32 * TILE_SIZE, 0.0, 0.0),
            Vec3::splat(1.0),
        ));
    }
    commands
        .spawn()
        .insert(Name::new("AsciiText"))
        //Needs transforms for parent heirarchy system to work
        .insert(Transform {
            translation: left_center,
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .push_children(&sprites)
        .id()
}
