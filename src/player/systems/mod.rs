pub mod movement;
pub mod spawn;

use bevy::prelude::*;
use movement::player_movement;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_movement);
}
