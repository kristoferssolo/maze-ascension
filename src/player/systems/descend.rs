use bevy::prelude::*;

use crate::{
    floor::components::{CurrentFloor, Floor},
    maze::components::MazeConfig,
    player::{
        components::{CurrentPosition, Player},
        events::DescendPlayer,
    },
};

pub(super) fn descend_player(
    query: Query<&CurrentPosition, With<Player>>,
    maze_config_query: Query<(&MazeConfig, &Floor), With<CurrentFloor>>,
    mut event_writer: EventWriter<DescendPlayer>,
) {
    let Ok((config, floor)) = maze_config_query.get_single() else {
        warn!("No current floor configuration found");
        return;
    };

    for current_hex in query.iter() {
        if current_hex.0 == config.start_pos {
            event_writer.send(DescendPlayer {
                floor: *floor.decrease(),
            });
            return;
        }
    }
}
