use bevy::utils::Duration;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use rand::{thread_rng, Rng};

use crate::debug::ENABLE_INSPECTOR;
use crate::graphics::{AnimatedSprite, FacingDirection, GraphicsHandles, PlayerAnimations};
use crate::screen_fadeout::{create_fadeout, fadeout};
use crate::tilemap::{Door, ExitEvent, TileCollider, WildSpawn};
use crate::{AsciiSheet, GameState, TILE_SIZE};

#[derive(Clone, Inspectable)]
pub struct CombatEvent;

#[derive(Component, Inspectable)]
pub struct Player {
    pub current_direction: FacingDirection,
    speed: f32,
    hitbox_size: f32,
    just_moved: bool,
    pub active: bool,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
    min_time: f32,
    max_time: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_event::<CombatEvent>()
            .add_system(fadeout::<CombatEvent>)
            .add_system(start_combat)
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
            .add_system_set(
                SystemSet::on_exit(GameState::Overworld)
                    .with_system(hide_player)
                    .with_system(reset_input),
            );
        if ENABLE_INSPECTOR {
            app.register_inspectable::<Player>()
                .register_inspectable::<CombatEvent>()
                .register_type::<EncounterTracker>();
        }
    }
}

fn start_combat(mut combat_event: EventReader<CombatEvent>, mut state: ResMut<State<GameState>>) {
    if let Some(_event) = combat_event.iter().next() {
        state
            .set(GameState::Combat)
            .expect("Failed to change state");
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
    if !player.active {
        return;
    }

    let to_move = player.speed * time.delta_seconds() * TILE_SIZE;

    let mut target_y = 0.0;
    if keyboard.pressed(KeyCode::W) {
        player.current_direction = FacingDirection::Up;
        target_y = to_move;
    }
    if keyboard.pressed(KeyCode::S) {
        player.current_direction = FacingDirection::Down;
        target_y = -to_move;
    }

    let mut target_x = 0.0;
    if keyboard.pressed(KeyCode::A) {
        player.current_direction = FacingDirection::Left;
        target_x = -to_move;
    }
    if keyboard.pressed(KeyCode::D) {
        player.current_direction = FacingDirection::Right;
        target_x = to_move;
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
    mut player_query: Query<(&Player, &mut EncounterTracker, &Transform)>,
    wall_query: Query<(&Transform, &WildSpawn), Without<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    ascii: Res<AsciiSheet>, //mut exit_event: EventWriter<ExitEvent>,
) {
    let (player, mut encounter, player_transform) = player_query.single_mut();
    if !player.just_moved {
        return;
    }

    for (spawn_transform, _) in wall_query.iter() {
        let collision = collide(
            player_transform.translation,
            Vec2::splat(TILE_SIZE * player.hitbox_size),
            spawn_transform.translation,
            Vec2::splat(TILE_SIZE),
        );

        if collision.is_some() {
            encounter.timer.tick(time.delta());
            break;
        }
    }

    if encounter.timer.just_finished() {
        encounter.timer.reset();
        //Get random time for next spawn
        let mut rng = thread_rng();
        let next_time: f32 = rng.gen_range(encounter.min_time..encounter.max_time);
        encounter
            .timer
            .set_duration(Duration::from_secs_f32(next_time));
        create_fadeout(&mut commands, ascii.clone(), CombatEvent, 0.3);
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
    mut player_query: Query<&mut Visibility, With<Player>>,
    //Seperate children query to handle players without children
    player_children_query: Query<&Children, With<Player>>,
    mut child_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut visibility = player_query.single_mut();
    visibility.is_visible = true;

    for children in player_children_query.iter() {
        for child in children.iter() {
            if let Ok(mut child_visibility) = child_query.get_mut(*child) {
                child_visibility.is_visible = true;
            }
        }
    }
}

fn door_collision(
    mut player_query: Query<(&mut Player, &Transform)>,
    wall_query: Query<(&Transform, &Door), Without<Player>>,
    mut commands: Commands,
    ascii: Res<AsciiSheet>, //mut exit_event: EventWriter<ExitEvent>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    if !player.active {
        return;
    }

    for (door_trans, door) in wall_query.iter() {
        //println!("Checking door");
        let collision = collide(
            player_transform.translation,
            Vec2::splat(TILE_SIZE * player.hitbox_size),
            door_trans.translation,
            Vec2::splat(TILE_SIZE),
        );

        if collision.is_some() {
            player.active = false;
            create_fadeout(&mut commands, ascii.clone(), ExitEvent(door.clone()), 0.3);
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    graphics: Res<GraphicsHandles>,
    animations: Res<PlayerAnimations>,
) {
    let mut sprite = TextureAtlasSprite::new(animations.walk_down[0]);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.characters.clone(),
            transform: Transform {
                translation: Vec3::new(12.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            current_direction: FacingDirection::Down,
            speed: 6.0,
            hitbox_size: 0.90,
            just_moved: false,
            active: true,
        })
        .insert(AnimatedSprite {
            current_frame: 0,
            timer: Timer::from_seconds(0.2, true),
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.0, true),
            min_time: 0.5,
            max_time: 2.5,
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
