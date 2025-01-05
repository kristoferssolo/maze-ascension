//! Floor transition handling system.
//!
//! This module manages player transitions between different maze floors,
//! handling both ascending and descending movements based on player position
//! and input.

use crate::{
    floor::{
        components::{CurrentFloor, Floor},
        events::TransitionFloor,
    },
    maze::components::MazeConfig,
    player::components::{CurrentPosition, Player},
};

use bevy::prelude::*;

/// Handles floor transitions when a player reaches start/end positions.
///
/// System checks if the player is at a valid transition point (start or end position)
/// and triggers floor transitions when the appropriate input is received.
pub fn handle_floor_transition(
    mut player_query: Query<&CurrentPosition, With<Player>>,
    maze_query: Query<(&MazeConfig, &Floor), With<CurrentFloor>>,
    mut event_writer: EventWriter<TransitionFloor>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok((config, floor)) = maze_query.get_single() else {
        warn!("Failed to get maze configuration for current floor - cannot ascend/descend player.");
        return;
    };

    for current_hex in player_query.iter_mut() {
        // Check for ascending (at end position)
        if current_hex.0 == config.end_pos {
            info!("Ascending");
            event_writer.send(TransitionFloor::Ascend);
        }

        // Check for descending (at start position, not on first floor)
        if current_hex.0 == config.start_pos && floor.0 != 1 {
            info!("Descending");
            event_writer.send(TransitionFloor::Descend);
        }
    }
}
