mod ascend;
mod descend;
mod input;
mod movement;
pub mod setup;
mod transition_floor;

use crate::maze::MazePluginLoaded;
use ascend::ascend_player;
use bevy::prelude::*;
use descend::descend_player;
use input::player_input;
use movement::player_movement;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (player_input, player_movement, ascend_player, descend_player)
            .chain()
            .run_if(resource_exists::<MazePluginLoaded>),
    );
}
