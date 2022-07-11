use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


use crate::graphics::GraphicsHandles;
use crate::player::Player;
use crate::screen_fadeout::fadeout;
use crate::GameState;
use crate::TILE_SIZE;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Map;

#[derive(Component, Clone, Inspectable)]
pub struct Door {
    pub path: String,
    pub new_x: i32,
    pub new_y: i32,
}

//TODO add direction from collision
#[derive(Clone, Inspectable)]
pub struct ExitEvent(pub Door);

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
//TODO needs some stats
pub struct WildSpawn;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExitEvent>()
            .add_system(load_exit)
            .add_system(fadeout::<ExitEvent>)
            .add_startup_system(spawn_sample_map)
            .add_system_set(SystemSet::on_exit(GameState::Overworld).with_system(hide_map))
            .add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(show_map));
    }
}

fn show_map(mut map_query: Query<&Children, With<Map>>, mut child_query: Query<&mut Visibility>) {
    let children = map_query.single_mut();
    for child in children.iter() {
        if let Ok(mut child_visibility) = child_query.get_mut(*child) {
            child_visibility.is_visible = true;
        }
    }
}

fn hide_map(mut map_query: Query<&Children, With<Map>>, mut child_query: Query<&mut Visibility>) {
    let children = map_query.single_mut();
    for child in children.iter() {
        if let Ok(mut child_visibility) = child_query.get_mut(*child) {
            child_visibility.is_visible = false;
        }
    }
}

fn spawn_sample_map(commands: Commands, graphics: Res<GraphicsHandles>) {
    load_map(commands, graphics, Path::new("assets/map.txt"));
}

fn load_exit(
    mut commands: Commands,
    graphics: Res<GraphicsHandles>,
    map_query: Query<Entity, With<Map>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    mut exit_event: EventReader<ExitEvent>,
) {
    if let Some(event) = exit_event.iter().next() {
        println!("Loading: {}", event.0.path);
        //Unload current map
        if !map_query.is_empty() {
            let map = map_query.single();
            commands.entity(map).despawn_recursive();
        }
        load_map(commands, graphics, Path::new(&event.0.path));
        let (mut transform, _) = player_query.single_mut();
        transform.translation.x = TILE_SIZE * event.0.new_x as f32;
        transform.translation.y = -TILE_SIZE * event.0.new_y as f32;
    }
}

fn load_map(mut commands: Commands, graphics: Res<GraphicsHandles>, path: &Path) {
    let input = File::open(path).expect("No map found");
    let mut tiles = Vec::new();
    let mut exits = VecDeque::new();

    let mut comment_counter = 0;
    for (y, line) in BufReader::new(input).lines().enumerate() {
        if let Ok(line) = line {
            for (x, c) in line.chars().enumerate() {
                if c == '/' {
                    // comment
                    comment_counter += 1;
                    parse_comment(&line, &mut exits);
                    break;
                } else {
                    tiles.push(parse_tile(
                        &mut commands,
                        &graphics,
                        c,
                        x as f32,
                        -(y as f32) + comment_counter as f32,
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

fn parse_comment(line: &str, exits: &mut VecDeque<Door>) {
    let comment: String = line.chars().skip(1).collect();
    let words: Vec<&str> = comment.split(' ').collect();

    let path = words[0];
    let x = words[1]
        .parse::<i32>()
        .expect("Bad comment formatting, no x coord");
    let y = words[2]
        .parse::<i32>()
        .expect("Bad comment formatting, no y coord");
    //TODO check if path actually exists
    exits.push_back(Door {
        path: path.to_string(),
        new_x: x,
        new_y: y,
    });
}

fn parse_tile(
    commands: &mut Commands,
    graphics: &GraphicsHandles,
    c: char,
    x: f32,
    y: f32,
    exits: &mut VecDeque<Door>,
) -> Entity {
    let tile = sprite_lookup(c);

    let tile_ent = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: tile,
            texture_atlas: graphics.tiles.clone(),
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
        'G' => {
            commands.entity(tile_ent).insert(WildSpawn);
        }
        'D' => {
            commands.entity(tile_ent).insert(
                exits
                    .pop_front()
                    .expect("More doors in map than listed scenes"),
            );
        }
        _ => {}
    }

    tile_ent
}

fn sprite_lookup(c: char) -> TextureAtlasSprite {
    let mut tile = TextureAtlasSprite::new(match c {
        '#' => 0,
        '.' => 11,
        'W' => 4,
        'D' => 8 * 6,
        'G' => 8 * 8 + 1,
        'R' => 8,
        'T' => 11 * 8 + 1,
        '@' => 9 * 8 + 5,
        _ => 8 * 4 + 5,
    });
    tile.custom_size = Some(Vec2::splat(TILE_SIZE));
    tile
}
