use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::tilemap::{Door, ExitEvent, TileCollider, WildSpawn};
use crate::{AsciiSheet, TILE_SIZE};

#[derive(Component)]
pub struct Player {
    speed: f32,
    hitbox_size: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(basic_player_movement.label("movement"))
            //If the wall collision happens first it pushes the player away and the door never collides
            //This is a race condition that very slightly changes gameplay
            .add_system(wall_collision.label("movement1").after("movement"))
            .add_system(door_collision.after("movement1"))
            .add_system(grass_collision.after("movement"));
    }
}

fn basic_player_movement(
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

fn grass_collision(
    player_query: Query<(&Player, &Transform)>,
    wall_query: Query<(&Transform, &WildSpawn), Without<Player>>,
    //spawn encounter event?
) {
    let (player, player_transform) = player_query.single();

    for (spawn_transform, _) in wall_query.iter() {
        //println!("Checking door");
        let collision = collide(
            player_transform.translation,
            Vec2::splat(TILE_SIZE * player.hitbox_size),
            spawn_transform.translation,
            Vec2::splat(TILE_SIZE),
        );

        if collision.is_some() {
            println!("Battle Start !");
        }
    }
}

fn door_collision(
    player_query: Query<(&Player, &Transform)>,
    wall_query: Query<(&Transform, &Door), Without<Player>>,
    mut exit_event: EventWriter<ExitEvent>,
) {
    let (player, player_transform) = player_query.single();

    for (door_trans, door) in wall_query.iter() {
        //println!("Checking door");
        let collision = collide(
            player_transform.translation,
            Vec2::splat(TILE_SIZE * player.hitbox_size),
            door_trans.translation,
            Vec2::splat(TILE_SIZE),
        );

        if collision.is_some() {
            exit_event.send(ExitEvent(door.clone()));
        }
    }
}

fn wall_collision(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (Without<Player>, With<TileCollider>)>,
    time: Res<Time>,
) {
    let (player, mut player_transform) = player_query.single_mut();

    for wall_trans in wall_query.iter() {
        let collision = collide(
            player_transform.translation,
            Vec2::splat(TILE_SIZE * player.hitbox_size),
            wall_trans.translation,
            Vec2::splat(TILE_SIZE),
        );

        let kick_back = player.speed * time.delta_seconds();
        if let Some(collision) = collision {
            match collision {
                Collision::Top => player_transform.translation.y += kick_back,
                Collision::Bottom => player_transform.translation.y -= kick_back,
                Collision::Left => player_transform.translation.x -= kick_back,
                Collision::Right => player_transform.translation.x += kick_back,
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
        .insert(Player {
            speed: 0.3,
            hitbox_size: 0.95,
        })
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
