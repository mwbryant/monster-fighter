use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::tilemap::{Door, ExitEvent, TileCollider, WildSpawn};
use crate::{AsciiSheet, GameState, TILE_SIZE};

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    hitbox_size: f32,
    just_moved: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .register_inspectable::<Player>()
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                    .with_system(basic_player_movement.label("movement"))
                    .with_system(door_collision.after("movement"))
                    .with_system(grass_collision.after("movement"))
                    .with_system(camera_follow),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Overworld)
                    .with_system(show_player)
                    .with_system(reset_input),
            )
            .add_system_set(SystemSet::on_exit(GameState::Overworld).with_system(hide_player));
    }
}

fn basic_player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (Without<Player>, With<TileCollider>)>,
) {
    let (mut player, mut transform) = player_query.single_mut();
    player.just_moved = false;

    let to_move = player.speed * time.delta_seconds() * TILE_SIZE;

    let mut target_x = 0.0;
    if keyboard.pressed(KeyCode::A) {
        target_x = -to_move;
    }
    if keyboard.pressed(KeyCode::D) {
        target_x = to_move;
    }

    let mut target_y = 0.0;
    if keyboard.pressed(KeyCode::W) {
        target_y = to_move;
    }
    if keyboard.pressed(KeyCode::S) {
        target_y = -to_move;
    }

    //Check if x movement is valid
    let target = transform.translation + Vec3::new(target_x, 0.0, 0.0);
    if wall_collision_check(target, &player, &wall_query) {
        transform.translation = target;
        if target_x != 0.0 {
            player.just_moved = true;
        }
    }

    //Check if y movement is valid
    let target = transform.translation + Vec3::new(0.0, target_y, 0.0);
    if wall_collision_check(target, &player, &wall_query) {
        transform.translation = target;
        if target_y != 0.0 {
            player.just_moved = true;
        }
    }
}

//Hack : https://github.com/bevyengine/bevy/issues/1700#issuecomment-803356041
// https://bevy-cheatbook.github.io/programming/states.html#with-input
fn reset_input(mut keyboard_input: ResMut<Input<KeyCode>>) {
    keyboard_input.clear();
}

fn wall_collision_check(
    target_player_pos: Vec3,
    player: &Player,
    wall_query: &Query<&Transform, (Without<Player>, With<TileCollider>)>,
) -> bool {
    for wall_trans in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * player.hitbox_size),
            wall_trans.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

fn grass_collision(
    player_query: Query<(&Player, &Transform)>,
    wall_query: Query<(&Transform, &WildSpawn), Without<Player>>,
    mut state: ResMut<State<GameState>>,
) {
    let (player, player_transform) = player_query.single();
    if !player.just_moved {
        return;
    }

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
            state
                .set(GameState::Combat)
                .expect("Failed to change state");
            break;
        }
    }
}

fn hide_player(
    mut player_query: Query<(&Children, &mut Visibility), With<Player>>,
    mut child_query: Query<&mut Visibility, Without<Player>>,
) {
    let (children, mut visibility) = player_query.single_mut();
    visibility.is_visible = false;
    for child in children.iter() {
        if let Ok(mut child_visibility) = child_query.get_mut(*child) {
            child_visibility.is_visible = false;
        }
    }
}

fn show_player(
    mut player_query: Query<(&Children, &mut Visibility), With<Player>>,
    mut child_query: Query<&mut Visibility, Without<Player>>,
) {
    let (children, mut visibility) = player_query.single_mut();
    visibility.is_visible = true;
    for child in children.iter() {
        if let Ok(mut child_visibility) = child_query.get_mut(*child) {
            child_visibility.is_visible = true;
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

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = Color::rgb(0.3, 0.3, 0.9);

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(12.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 6.0,
            hitbox_size: 0.90,
            just_moved: false,
        })
        //Background sprite
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                sprite: background_sprite,
                texture_atlas: ascii.0.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -1.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<(&Player, &Transform)>,
) {
    let mut cam_transform = camera_query.single_mut();
    let (_, player_transform) = player_query.single();

    cam_transform.translation.x = player_transform.translation.x;
    cam_transform.translation.y = player_transform.translation.y;
}
