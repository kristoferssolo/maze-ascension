use crate::{
    floor::{components::CurrentFloor, events::TransitionFloor},
    maze::components::MazeConfig,
    player::components::{CurrentPosition, Player},
};
use bevy::prelude::*;

pub(super) fn ascend_player(
    query: Query<&CurrentPosition, With<Player>>,
    maze_config_query: Query<&MazeConfig, With<CurrentFloor>>,
    mut event_writer: EventWriter<TransitionFloor>,
) {
    let Ok(config) = maze_config_query.get_single() else {
        warn!("Failed to get maze configuration for current floor - cannot ascend player");
        return;
    };

    for current_hex in query.iter() {
        if current_hex.0 == config.end_pos {
            event_writer.send(TransitionFloor::Ascend);
            return;
        }
    }
}
