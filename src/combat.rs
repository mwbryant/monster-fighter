use crate::ascii_text::spawn_ascii_text;
use crate::nine_sprite::{spawn_nine_sprite, NineSprite, NineSpriteIndices};
use crate::{AsciiSheet, GameState, RESOLUTION, TILE_SIZE};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Inspectable)]
enum CombatMenuType {
    Fight = 0,
    Swap,
    Item,
    Run,
}

#[derive(Component, Inspectable)]
struct CombatMenu {
    selected: CombatMenuType,
}

#[derive(Component, Inspectable)]
struct CombatMenuButton {
    id: CombatMenuType,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Combat).with_system(exit_combat))
            .register_inspectable::<CombatMenuButton>()
            .register_inspectable::<CombatMenu>()
            .add_system_set(
                SystemSet::on_enter(GameState::Combat)
                    .with_system(center_camera)
                    .with_system(create_combat_menu),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Combat)
                    .with_system(highlight_selected_menu)
                    .with_system(combat_menu_input),
            )
            .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(delete_combat_menu));
    }
}
fn combat_menu_input(mut menu_query: Query<&mut CombatMenu>, keyboard: Res<Input<KeyCode>>) {
    let mut menu = menu_query.single_mut();
    let mut to_select = menu.selected.clone() as isize;

    if keyboard.just_pressed(KeyCode::D) {
        to_select += 1;
    }
    if keyboard.just_pressed(KeyCode::A) {
        to_select -= 1;
    }
    if keyboard.just_pressed(KeyCode::W) {
        to_select += 2;
    }
    if keyboard.just_pressed(KeyCode::S) {
        to_select -= 2;
    }

    // add 4 to force positive outcome
    to_select = (to_select + 4) % 4;

    menu.selected = match to_select {
        0 => CombatMenuType::Fight,
        1 => CombatMenuType::Swap,
        2 => CombatMenuType::Item,
        3 => CombatMenuType::Run,
        _ => unreachable!("Bad menu selection!"),
    };
}

fn highlight_selected_menu(
    menu_query: Query<&CombatMenu>,
    button_query: Query<(&CombatMenuButton, &Children)>,
    nine_sprite_query: Query<(&NineSprite, &Children)>,
    mut child_query: Query<&mut TextureAtlasSprite>,
) {
    //TODO this weird mess needs a lot of work...
    //How to climb and bevy hierarchy more effectively and genericly
    let menu = menu_query.single();

    for (element, children) in button_query.iter() {
        //Highlight this buttons nine sprite
        for &button_child in children.iter() {
            if let Ok((_, nine_sprite_children)) = nine_sprite_query.get(button_child) {
                //Highlight all children of the nine sprite red
                for &child in nine_sprite_children.iter() {
                    if let Ok(mut child_sprite) = child_query.get_mut(child) {
                        //Only highlight if id is the selected option
                        if element.id == menu.selected {
                            (*child_sprite).color = Color::RED;
                        } else {
                            (*child_sprite).color = Color::WHITE;
                        }
                    }
                }
            }
        }
    }
}
fn create_combat_button(
    commands: &mut Commands,
    ascii: AsciiSheet,
    indices: NineSpriteIndices,
    translation: Vec3,
    text: &str,
    id: CombatMenuType,
    box_width: f32,
    box_height: f32,
) -> Entity {
    //XXX why -3 tiles
    let text_offset = Vec3::new(-(box_width - 3.0 * TILE_SIZE) / 2.0, 0.0, 0.0);

    let button = commands
        .spawn()
        .insert(Name::new(text.to_owned() + "Button"))
        .insert(CombatMenuButton { id: id })
        .insert(Transform {
            translation: translation,
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .id();
    let sprite = spawn_nine_sprite(commands, ascii.clone(), indices, box_width, box_height);
    let text = spawn_ascii_text(commands, ascii.clone(), text, text_offset);
    commands.entity(button).push_children(&[sprite, text]);
    button
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

    let run = create_combat_button(
        &mut commands,
        ascii.clone(),
        *indices,
        Vec3::new(right_offset, bottom_offset, 0.0),
        "Run",
        CombatMenuType::Run,
        box_width,
        box_height,
    );

    let item = create_combat_button(
        &mut commands,
        ascii.clone(),
        *indices,
        Vec3::new(right_offset - box_width, bottom_offset, 0.0),
        "Item",
        CombatMenuType::Item,
        box_width,
        box_height,
    );

    let swap = create_combat_button(
        &mut commands,
        ascii.clone(),
        *indices,
        Vec3::new(right_offset, bottom_offset + box_height, 0.0),
        "Swap",
        CombatMenuType::Swap,
        box_width,
        box_height,
    );

    let fight = create_combat_button(
        &mut commands,
        ascii.clone(),
        *indices,
        Vec3::new(right_offset - box_width, bottom_offset + box_height, 0.0),
        "Fight",
        CombatMenuType::Fight,
        box_width,
        box_height,
    );

    commands
        .spawn()
        .insert(Name::new("CombatMenu"))
        .insert(CombatMenu {
            selected: CombatMenuType::Fight,
        })
        //Needs transforms for parent heirarchy system to work
        .insert(Transform::default())
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
