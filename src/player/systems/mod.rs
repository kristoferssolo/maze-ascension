mod input;
mod movement;
pub mod setup;
mod vertical_transition;

use crate::{screens::Screen, AppSet};
use bevy::prelude::*;
use input::player_input;
use movement::player_movement;
use vertical_transition::handle_floor_transition;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_input.in_set(AppSet::RecordInput),
            player_movement,
            handle_floor_transition.in_set(AppSet::RecordInput),
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
}
