use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::tilemap::{Door, ExitEvent, TileCollider};
use crate::{AsciiSheet, TILE_SIZE};

#[derive(Component)]
pub struct Player {
    speed: f32,
}

pub fn basic_player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = player_query.single_mut();
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * time.delta_seconds();
    }
}

pub fn wall_collision(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<(&TileCollider, &Transform, Option<&Door>), Without<Player>>,
    mut exit_event: EventWriter<ExitEvent>,
    time: Res<Time>,
) {
    let (player, mut player_transform) = player_query.single_mut();
    let player_size_decrease = 0.95;

    for (_, wall_trans, door) in wall_query.iter() {
        let collision = collide(
            player_transform.translation,
            Vec2::splat(TILE_SIZE * player_size_decrease),
            wall_trans.translation,
            Vec2::splat(TILE_SIZE),
        );

        if collision.is_some() && door.is_some() {
            println!("EXIT");
            let door = door.unwrap();
            exit_event.send(ExitEvent(door.0.clone()));
        }

        if let Some(collision) = collision {
            match collision {
                Collision::Top => {
                    player_transform.translation.y += player.speed * time.delta_seconds()
                }
                Collision::Bottom => {
                    player_transform.translation.y -= player.speed * time.delta_seconds()
                }
                Collision::Left => {
                    player_transform.translation.x -= player.speed * time.delta_seconds()
                }
                Collision::Right => {
                    player_transform.translation.x += player.speed * time.delta_seconds()
                }
            }
        }
    }
}

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = Color::rgb(0.3, 0.3, 0.9);

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player { speed: 0.3 })
        .id();

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: background_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands.entity(player).push_children(&[background]);
}
