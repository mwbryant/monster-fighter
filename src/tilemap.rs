use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::AsciiSheet;
use crate::TILE_SIZE;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct TileCollider;

pub fn spawn_sample_map(commands: Commands, ascii: Res<AsciiSheet>) {
    load_map(commands, ascii, Path::new("assets/map.txt"));
}

fn load_map(mut commands: Commands, ascii: Res<AsciiSheet>, path: &Path) {
    let input = File::open(path).expect("No map found");
    for (y, line) in BufReader::new(input).lines().enumerate() {
        if let Ok(line) = line {
            for (x, c) in line.chars().enumerate() {
                parse_tile(&mut commands, &ascii, c, x as f32, -(y as f32));
            }
        }
    }
}

fn parse_tile(commands: &mut Commands, ascii: &AsciiSheet, c: char, x: f32, y: f32) {
    let mut tile = match c {
        '#' => {
            let mut sprite = TextureAtlasSprite::new('#' as usize);
            sprite.color = Color::rgb(0.8, 0.8, 0.8);
            sprite
        }
        '.' => {
            let mut sprite = TextureAtlasSprite::new('.' as usize);
            sprite.color = Color::rgb(0.1, 0.1, 0.1);
            sprite
        }
        'W' => {
            let mut sprite = TextureAtlasSprite::new('#' as usize);
            sprite.color = Color::rgb(0.7, 0.4, 0.1);
            sprite
        }
        'D' => {
            let mut sprite = TextureAtlasSprite::new(14 * 16 + 9); // weird door sprite
            sprite.color = Color::rgb(0.7, 0.4, 0.4);
            sprite
        }
        'G' => {
            let mut sprite = TextureAtlasSprite::new(15 * 16 + 7); // weird door sprite
            sprite.color = Color::rgb(0.2, 0.9, 0.2);
            sprite
        }
        'R' => {
            let mut sprite = TextureAtlasSprite::new('#' as usize); // weird door sprite
            sprite.color = Color::rgb(0.9, 0.2, 0.2);
            sprite
        }
        _ => {
            let mut sprite = TextureAtlasSprite::new('X' as usize); // weird door sprite
            sprite.color = Color::rgb(0.9, 0.0, 0.9);
            sprite
        }
    };
    tile.custom_size = Some(Vec2::splat(TILE_SIZE));

    let tile_ent = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: tile,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(TILE_SIZE * x, TILE_SIZE * y, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    match c {
        '#' | 'W' => {
            commands.entity(tile_ent).insert(TileCollider);
        }
        _ => {}
    }
}
