use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::{thread_rng, Rng};

//FIXME this is a horrible
pub const HEALTH_UI_ID: usize = 2;

use crate::{
    ascii::{spawn_ascii_sprite, spawn_ascii_text, AsciiSheet},
    TILE_SIZE,
};

#[derive(Inspectable, Copy, Clone)]
pub enum EnemyType {
    Bat,
    Zombie,
    Ghost,
    Demon,
    Giant,
}

#[derive(Component, Inspectable, Copy, Clone)]
pub struct Enemy {
    enemy_type: EnemyType,
    sprite_index: usize,
    color: Color,
    pub health: i64,
    exp: i64,
}

fn get_random_enemy() -> Enemy {
    let mut rng = thread_rng();
    //TODO weighted odds
    let rand = rng.gen_range(0..5);
    match rand {
        0 => Enemy {
            enemy_type: EnemyType::Bat,
            sprite_index: 'b' as usize,
            health: 3,
            color: Color::rgb(0.6, 0.6, 0.6),
            exp: 3,
        },
        1 => Enemy {
            enemy_type: EnemyType::Zombie,
            sprite_index: 'Z' as usize,
            health: 5,
            color: Color::rgb(0.6, 1.0, 0.6),
            exp: 7,
        },
        2 => Enemy {
            enemy_type: EnemyType::Ghost,
            sprite_index: 'g' as usize,
            health: 6,
            color: Color::rgb(0.9, 0.9, 0.9),
            exp: 7,
        },
        3 => Enemy {
            enemy_type: EnemyType::Demon,
            sprite_index: 'D' as usize,
            health: 10,
            color: Color::rgb(0.9, 0.2, 0.2),
            exp: 15,
        },
        4 => Enemy {
            enemy_type: EnemyType::Giant,
            sprite_index: 'G' as usize,
            health: 20,
            color: Color::rgb(0.1, 0.5, 0.1),
            exp: 45,
        },
        _ => {
            unreachable!("Bad enemy roll");
        }
    }
}

pub fn create_enemy(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let enemy = get_random_enemy();

    let sprite = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        //FIXME find a better way to generate enemies pls
        enemy.sprite_index,
        enemy.color,
        Vec3::new(0.0, 0.5, 1.0),
        Vec3::splat(3.0),
    );
    //TODO center
    let health_bar = spawn_ascii_text(
        &mut commands,
        ascii.clone(),
        &format!("Health: {}", enemy.health),
        Vec3::new(-0.5, 1.0 - 2.0 * TILE_SIZE, 1.0),
        HEALTH_UI_ID,
    );
    commands
        .spawn()
        .insert(enemy)
        .insert(Name::new("Enemy"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&[sprite, health_bar]);
}

pub fn destroy_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
