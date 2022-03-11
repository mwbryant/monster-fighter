use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{debug::ENABLE_INSPECTOR, player::Player};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_graphics)
            .add_system(animate_player)
            .add_system(animate_sprites);
        if ENABLE_INSPECTOR {
            app.register_type::<AnimatedSprite>();
        }
    }
}

pub struct GraphicsHandles {
    pub characters: Handle<TextureAtlas>,
}

#[derive(Component, Inspectable)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct PlayerAnimations {
    pub walk_down: Vec<usize>,
    pub walk_up: Vec<usize>,
    pub walk_left: Vec<usize>,
    pub walk_right: Vec<usize>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AnimatedSprite {
    pub current_frame: usize,
    pub timer: Timer,
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("characters.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(16.0), 12, 9, Vec2::splat(1.0));
    let atlas_handle = atlases.add(atlas);

    commands.insert_resource(GraphicsHandles {
        characters: atlas_handle,
    });

    commands.insert_resource(PlayerAnimations {
        walk_down: vec![3, 4, 5],
        walk_up: vec![39, 40, 41],
        walk_left: vec![15, 16, 17],
        walk_right: vec![27, 28, 29],
    });
}

//TODO restructure this to support animations more generally
fn animate_sprites(mut sprites: Query<&mut AnimatedSprite>, time: Res<Time>) {
    for mut sprite in sprites.iter_mut() {
        sprite.timer.tick(time.delta());
        if sprite.timer.just_finished() {
            //Probs not dangerous but
            //FIXME handle wrap around
            sprite.current_frame += 1;
        }
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlasSprite, &AnimatedSprite, &Player)>,
    animations: Res<PlayerAnimations>,
) {
    let (mut sprite, animated_sprite, player) = player_query.single_mut();
    //FIXME clean this up
    match player.current_direction {
        FacingDirection::Up => {
            sprite.index =
                animations.walk_up[animated_sprite.current_frame % animations.walk_up.len()];
        }
        FacingDirection::Down => {
            sprite.index =
                animations.walk_down[animated_sprite.current_frame % animations.walk_down.len()];
        }
        FacingDirection::Left => {
            sprite.index =
                animations.walk_left[animated_sprite.current_frame % animations.walk_left.len()];
        }
        FacingDirection::Right => {
            sprite.index =
                animations.walk_right[animated_sprite.current_frame % animations.walk_right.len()];
        }
    }
}
