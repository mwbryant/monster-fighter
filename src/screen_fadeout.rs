use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::player::Player;

#[derive(Component, Inspectable)]
pub struct ScreenFade<T> {
    pub alpha: f32,
    pub sent: bool,
    //Fires when screen is completely black
    pub event: T,
}

pub fn fadeout<T: 'static + std::marker::Send + std::marker::Sync + Clone>(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut fade_query: Query<(
        Entity,
        &mut ScreenFade<T>,
        &mut Timer,
        &mut TextureAtlasSprite,
    )>,
    time: Res<Time>,
    mut event: EventWriter<T>,
) {
    let fade = fade_query.get_single_mut();
    let mut player = player_query.single_mut();
    if let Ok((entity, mut fade, mut timer, mut sprite)) = fade {
        player.active = false;
        timer.tick(time.delta());

        if timer.percent() < 0.5 {
            fade.alpha = timer.percent() * 2.0;
        } else {
            fade.alpha = timer.percent_left() * 2.0;
        }
        sprite.color.set_a(fade.alpha);

        if timer.percent() > 0.5 && !fade.sent {
            event.send(fade.event.clone());
            fade.sent = true;
        }

        if timer.just_finished() {
            commands.entity(entity).despawn_recursive();
            player.active = true;
        }
    }
}
