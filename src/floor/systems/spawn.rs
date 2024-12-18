use bevy::prelude::*;

use crate::{floor::events::TransitionFloor, maze::events::SpawnMaze};

pub(super) fn spawn_floor(mut commands: Commands, mut event_reader: EventReader<TransitionFloor>) {
    for event in event_reader.read() {
        let floor = event.floor;
        info!("Creating level for floor {}", floor);
        commands.trigger(SpawnMaze { floor, ..default() });
    }
}
