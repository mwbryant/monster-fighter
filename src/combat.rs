use crate::ascii_text::spawn_ascii_text;
use crate::nine_sprite::{spawn_nine_sprite, NineSpriteIndices};
use crate::{AsciiSheet, GameState, RESOLUTION, TILE_SIZE};
use bevy::prelude::*;

#[derive(Component)]
struct CombatMenu;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Combat).with_system(exit_combat))
            .add_system_set(
                SystemSet::on_enter(GameState::Combat)
                    .with_system(center_camera)
                    .with_system(create_combat_menu),
            )
            .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(delete_combat_menu));
    }
}

fn create_combat_menu(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    indices: Res<NineSpriteIndices>,
) {
    let box_width = 7.0 * TILE_SIZE;
    let box_height = 3.0 * TILE_SIZE;
    let bottom_offset = -1.0 + box_height / 2.0;
    let right_offset = 1.0 * RESOLUTION - box_width / 2.0;
    //XXX why -3 tiles
    let text_offset = Vec3::new(-(box_width - 3.0 * TILE_SIZE) / 2.0, 0.0, 0.0);

    let run = spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        box_width,
        box_height,
        Vec3::new(right_offset, bottom_offset, 0.0),
    );
    let run_text = spawn_ascii_text(&mut commands, ascii.clone(), "Run", text_offset);
    commands.entity(run).add_child(run_text);
    let item = spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        box_width,
        box_height,
        Vec3::new(right_offset - box_width, bottom_offset, 0.0),
    );
    let item_text = spawn_ascii_text(&mut commands, ascii.clone(), "Item", text_offset);
    commands.entity(item).add_child(item_text);
    let swap = spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        box_width,
        box_height,
        Vec3::new(right_offset, bottom_offset + box_height, 0.0),
    );
    let swap_text = spawn_ascii_text(&mut commands, ascii.clone(), "Swap", text_offset);
    commands.entity(swap).add_child(swap_text);
    let fight = spawn_nine_sprite(
        &mut commands,
        ascii.clone(),
        *indices,
        box_width,
        box_height,
        Vec3::new(right_offset - box_width, bottom_offset + box_height, 0.0),
    );
    let fight_text = spawn_ascii_text(&mut commands, ascii.clone(), "Fight", text_offset);
    commands.entity(fight).add_child(fight_text);

    commands
        .spawn()
        .insert(Name::new("CombatMenu"))
        .insert(CombatMenu)
        //Needs transforms for parent heirarchy system to work
        .insert(Transform {
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .push_children(&[fight, run, item, swap])
        .id();
}

fn delete_combat_menu(mut commands: Commands, mut menu_query: Query<Entity, With<CombatMenu>>) {
    let menu = menu_query.single_mut();
    commands.entity(menu).despawn_recursive();
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
