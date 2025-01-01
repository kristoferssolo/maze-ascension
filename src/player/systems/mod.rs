mod input;
mod movement;
pub mod setup;
mod vertical_transition;

use crate::maze::MazePluginLoaded;
use bevy::prelude::*;
use input::player_input;
use movement::player_movement;
use vertical_transition::handle_floor_transition;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_input,
            player_movement.after(player_input),
            handle_floor_transition,
        )
            .run_if(resource_exists::<MazePluginLoaded>),
    );
}
