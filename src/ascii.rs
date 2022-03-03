use crate::TILE_SIZE;
use bevy::prelude::*;

#[derive(Clone)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = color;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets.load("Ascii.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0),
    );
    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}

pub fn spawn_ascii_text(
    commands: &mut Commands,
    ascii: AsciiSheet,
    to_print: &str,
    left_center: Vec3,
) -> Entity {
    let color = Color::rgb(0.8, 0.8, 0.8);

    let mut sprites = Vec::new();
    for (i, char) in to_print.chars().enumerate() {
        sprites.push(spawn_ascii_sprite(
            commands,
            &ascii,
            char as usize,
            color,
            Vec3::new(i as f32 * TILE_SIZE, 0.0, 0.0),
            Vec3::splat(1.0),
        ));
    }
    commands
        .spawn()
        .insert(Name::new("AsciiText"))
        //Needs transforms for parent heirarchy system to work
        .insert(Transform {
            translation: left_center,
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .push_children(&sprites)
        .id()
}
