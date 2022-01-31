use bevy::prelude::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::AsciiSheet;
use crate::TILE_SIZE;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Door(pub String);

pub struct ExitEvent(pub String);

#[derive(Component)]
pub struct TileCollider;

pub fn spawn_sample_map(commands: Commands, ascii: Res<AsciiSheet>) {
    load_map(commands, ascii, Path::new("assets/map.txt"));
}

pub fn load_exit(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    map_query: Query<(Entity, &Map, &Children)>,
    mut exit_event: EventReader<ExitEvent>,
) {
    if let Some(event) = exit_event.iter().next() {
        //Unload current map
        if !map_query.is_empty() {
            println!("CLEARING MAP");
            //Clear children first to prevent orphans
            let (entity, _, children) = map_query.single();
            for child in children.iter() {
                commands.entity(*child).despawn();
            }
            commands.entity(entity).despawn();
        }
        load_map(commands, ascii, Path::new(&event.0));
    }
}

fn load_map(mut commands: Commands, ascii: Res<AsciiSheet>, path: &Path) {
    let input = File::open(path).expect("No map found");
    let mut tiles = Vec::new();
    let mut exits = VecDeque::new();

    for (y, line) in BufReader::new(input).lines().enumerate() {
        if let Ok(line) = line {
            for (x, c) in line.chars().enumerate() {
                if c == '/' {
                    // comment
                    parse_comment(&line, &mut exits);
                    break;
                } else {
                    tiles.push(parse_tile(
                        &mut commands,
                        &ascii,
                        c,
                        x as f32,
                        -(y as f32),
                        &mut exits,
                    ));
                }
            }
        }
    }
    //TODO assert exits is empty

    commands
        .spawn()
        .insert(Name::new("Map"))
        //Needs transforms for parent heirarchy system to work
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Map)
        .push_children(&tiles);
}

fn parse_comment(line: &str, exits: &mut VecDeque<String>) {
    let path = line.chars().skip(1).collect();
    //TODO check if path actually exists
    exits.push_back(path);
}

fn parse_tile(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    c: char,
    x: f32,
    y: f32,
    exits: &mut VecDeque<String>,
) -> Entity {
    let tile = sprite_lookup(c);

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
        'D' => {
            commands
                .entity(tile_ent)
                .insert(Door(
                    exits
                        .pop_front()
                        .expect("More doors in map than listed scenes"),
                ))
                .insert(TileCollider);
        }
        _ => {}
    }

    tile_ent
}

fn sprite_lookup(c: char) -> TextureAtlasSprite {
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
    tile
}
