use crate::{
    audio::SoundEffect,
    player::{
        assets::PlayerAssets,
        components::{MovementTarget, Player},
    },
};

use bevy::prelude::*;
use rand::seq::SliceRandom;

pub fn play_movement_sound(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    moving_players: Query<&MovementTarget, (Changed<MovementTarget>, With<Player>)>,
) {
    for movement_target in moving_players.iter() {
        if movement_target.is_none() {
            continue;
        }

        let rng = &mut rand::thread_rng();
        if let Some(random_step) = player_assets.steps.choose(rng) {
            commands.spawn((
                AudioPlayer(random_step.clone()),
                PlaybackSettings::DESPAWN,
                SoundEffect,
            ));
        }
    }
}
