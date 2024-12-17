mod input;
mod movement;
pub mod setup;

use bevy::prelude::*;
use input::player_input;
use movement::player_movement;

use crate::maze::MazePluginLoaded;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (player_input, player_movement)
            .chain()
            .run_if(resource_exists::<MazePluginLoaded>),
    );
}
