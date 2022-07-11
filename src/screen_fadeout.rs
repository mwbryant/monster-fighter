use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    player::Player,
};

#[derive(Component)]
pub struct ScreenFade<T> {
    pub alpha: f32,
    pub sent: bool,
    pub timer: Timer,
    //Fires when screen is completely black
    pub event: T,
}

//XXX how to assert that T has a system registered to match the fadeout
pub fn create_fadeout<T: 'static + std::marker::Send + std::marker::Sync + Clone>(
    commands: &mut Commands,
    ascii: AsciiSheet,
    event: T,
    fade_time: f32,
) {
    let screen_fade = spawn_ascii_sprite(
        commands,
        &ascii,
        0,
        Color::rgba(0.0, 0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 999.9),
        Vec3::splat(100.0),
    );
    commands
        .entity(screen_fade)
        .insert(ScreenFade {
            alpha: 0.0,
            sent: false,
            timer: Timer::from_seconds(fade_time, false),
            event: event,
        })
        .insert(Name::new("Fadeout"));
}

//T is the event to fire when the screen is compeletly black
pub fn fadeout<T: 'static + std::marker::Send + std::marker::Sync + Clone>(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut fade_query: Query<(
        Entity,
        &mut ScreenFade<T>,
        &mut TextureAtlasSprite,
    )>,
    time: Res<Time>,
    mut event: EventWriter<T>,
) {
    let mut player = player_query.single_mut();
    for (entity, mut fade, mut sprite) in fade_query.iter_mut() {
        player.active = false;
        fade.timer.tick(time.delta());

        if fade.timer.percent() < 0.5 {
            fade.alpha = fade.timer.percent() * 2.0;
        } else {
            fade.alpha = fade.timer.percent_left() * 2.0;
        }
        sprite.color.set_a(fade.alpha);

        if fade.timer.percent() > 0.5 && !fade.sent {
            event.send(fade.event.clone());
            fade.sent = true;
        }

        if fade.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
            player.active = true;
        }
    }
}
