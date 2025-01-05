use bevy::prelude::*;

use crate::{
    floor::{
        components::{CurrentFloor, Floor},
        events::TransitionFloor,
    },
    maze::components::MazeConfig,
    player::components::{CurrentPosition, Player, TranstitionState},
};

pub fn handle_floor_transition(
    mut player_query: Query<(&CurrentPosition, &mut TranstitionState), With<Player>>,
    maze_query: Query<(&MazeConfig, &Floor), With<CurrentFloor>>,
    mut event_writer: EventWriter<TransitionFloor>,
) {
    let Ok((config, floor)) = maze_query.get_single() else {
        warn!("Failed to get maze configuration for current floor - cannot ascend/descend player.");
        return;
    };

    for (current_hex, mut transition_state) in player_query.iter_mut() {
        // Reset transition state if moved to a new position
        if current_hex.0 != transition_state.last_position {
            transition_state.just_transitioned = false;
        }
        transition_state.last_position = current_hex.0;

        // Skip if transition just happened
        if transition_state.just_transitioned {
            continue;
        }

        // Check for ascending
        if current_hex.0 == config.end_pos {
            info!("Ascending");
            event_writer.send(TransitionFloor::Ascend);
            transition_state.just_transitioned = true;
        }

        // Check for descending
        if current_hex.0 == config.start_pos && floor.0 != 1 {
            info!("Descending");
            event_writer.send(TransitionFloor::Descend);
            transition_state.just_transitioned = true;
        }
    }
}
