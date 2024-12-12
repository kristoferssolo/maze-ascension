mod input;
mod movement;
pub mod spawn;

use bevy::prelude::*;
use input::player_input;
use movement::player_movement;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (player_input, player_movement.after(player_input)));
}