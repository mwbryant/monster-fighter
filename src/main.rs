use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowMode;

mod tilemap;

use tilemap::spawn_sample_map;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.10;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component, Clone)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct Player;

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Monster Fighter".to_string(),
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_sample_map)
        .add_system(camera_follow)
        .add_system(basic_player_movement)
        .run();
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

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    commands.spawn_bundle(camera);
}

fn basic_player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    let speed = 0.3;
    let (_, mut transform) = player_query.single_mut();
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= speed * time.delta_seconds();
    }
}

fn camera_follow(
    mut camera_query: Query<(&Camera, &mut Transform), Without<Player>>,
    player_query: Query<(&Player, &Transform)>,
) {
    let (_, mut cam_transform) = camera_query.single_mut();
    let (_, player_transform) = player_query.single();

    cam_transform.translation.x = player_transform.translation.x;
    cam_transform.translation.y = player_transform.translation.y;
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = Color::rgb(0.3, 0.3, 0.9);

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(2.0 * TILE_SIZE, -6.0 * TILE_SIZE, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
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
