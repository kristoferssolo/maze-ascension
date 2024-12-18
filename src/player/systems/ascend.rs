use bevy::prelude::*;

use crate::{
    floor::{
        components::{CurrentFloor, Floor},
        events::TransitionFloor,
    },
    maze::components::MazeConfig,
    player::components::{CurrentPosition, Player},
};

pub(super) fn ascend_player(
    query: Query<&CurrentPosition, With<Player>>,
    maze_config_query: Query<(&MazeConfig, &Floor), With<CurrentFloor>>,
    mut event_writer: EventWriter<TransitionFloor>,
) {
    let Ok((config, floor)) = maze_config_query.get_single() else {
        return;
    };

    for current_hex in query.iter() {
        if current_hex.0 == config.end_pos {
            event_writer.send(TransitionFloor {
                floor: *floor.increase(),
            });
            return;
        }
    }
}
