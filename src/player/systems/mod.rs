mod input;
mod movement;
pub mod setup;
mod sound_effect;
mod vertical_transition;

use crate::{screens::Screen, AppSet};
use bevy::prelude::*;
use input::player_input;
use movement::player_movement;
use sound_effect::play_movement_sound;
use vertical_transition::handle_floor_transition;

use super::assets::PlayerAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_input.in_set(AppSet::RecordInput),
            player_movement,
            handle_floor_transition.in_set(AppSet::RecordInput),
            (play_movement_sound)
                .chain()
                .run_if(resource_exists::<PlayerAssets>)
                .in_set(AppSet::Update),
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
}
