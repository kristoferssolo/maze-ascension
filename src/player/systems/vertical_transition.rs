use bevy::prelude::*;

use crate::{
    floor::{
        components::{CurrentFloor, Floor},
        events::TransitionFloor,
    },
    maze::components::MazeConfig,
    player::components::{CurrentPosition, Player},
};

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
        // Check for ascending
        if current_hex.0 == config.end_pos {
            info!("Ascending");
            event_writer.send(TransitionFloor::Ascend);
        }

        // Check for descending
        if current_hex.0 == config.start_pos && floor.0 != 1 {
            info!("Descending");
            event_writer.send(TransitionFloor::Descend);
        }
    }
}
