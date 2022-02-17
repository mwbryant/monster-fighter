use crate::{AsciiSheet, GameState};
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

fn enter_combat(mut commands: Commands, ascii: Res<AsciiSheet>) {
    //commands.spawn_bundle(SpriteSheetBundle {
    //sprite: sprite,
    //texture_atlas: ascii.0.clone(),
    //transform: Transform {
    //translation: Vec3::new(12.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    //..Default::default()
    //},
    //..Default::default()
    //});
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
