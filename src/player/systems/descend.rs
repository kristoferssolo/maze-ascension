use bevy::prelude::*;

use crate::{
    floor::{
        components::{CurrentFloor, Floor},
        events::TransitionFloor,
    },
    maze::components::MazeConfig,
    player::components::{CurrentPosition, Player},
};

pub(super) fn descend_player(
    query: Query<&CurrentPosition, With<Player>>,
    maze_config_query: Query<(&MazeConfig, &Floor), With<CurrentFloor>>,
    mut event_writer: EventWriter<TransitionFloor>,
) {
    let Ok((config, floor)) = maze_config_query.get_single() else {
        warn!("Failed to get maze configuration for current floor - cannot descend player");
        return;
    };

    for current_hex in query.iter() {
        if current_hex.0 == config.start_pos && floor.0 != 1 {
            event_writer.send(TransitionFloor::Descend);
            return;
        }
    }
}
