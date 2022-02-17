use crate::nine_sprite::{spawn_nine_sprite, NineSpriteIndices};
use crate::{AsciiSheet, GameState, TILE_SIZE};
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Combat).with_system(exit_combat))
            .add_system_set(
                SystemSet::on_enter(GameState::Combat)
                    .with_system(center_camera)
                    .with_system(enter_combat),
            );
    }
}

fn enter_combat(mut commands: Commands, ascii: Res<AsciiSheet>, indices: Res<NineSpriteIndices>) {
    spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        Vec3::new(4.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0),
    );
    spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        Vec3::new(7.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0),
    );
    spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        Vec3::new(4.0 * TILE_SIZE, -5.0 * TILE_SIZE, 0.0),
    );
    spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        Vec3::new(7.0 * TILE_SIZE, -5.0 * TILE_SIZE, 0.0),
    );
}

fn exit_combat(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Battle End !");
        state
            .set(GameState::Overworld)
            .expect("Failed to change state");
    }
}

fn center_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut cam_transform = camera_query.single_mut();
    cam_transform.translation.x = 0.0;
    cam_transform.translation.y = 0.0;
}
