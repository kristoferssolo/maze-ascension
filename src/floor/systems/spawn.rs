use crate::{
    floor::{
        components::{CurrentFloor, Floor},
        events::TransitionFloor,
    },
    maze::events::SpawnMaze,
};
use bevy::prelude::*;

pub(super) fn spawn_floor(
    mut commands: Commands,
    query: Query<&mut Floor, With<CurrentFloor>>,
    mut event_reader: EventReader<TransitionFloor>,
) {
    let Ok(floor) = query.get_single() else {
        return;
    };

    for event in event_reader.read() {
        let floor = match event {
            TransitionFloor::Ascend => *floor.increased(),
            TransitionFloor::Descend => *floor.decreased(),
        };

        if floor == 1 {
            warn!("Cannot descend below floor 1");
            return;
        }

        info!("Creating level for floor {}", floor);

        commands.trigger(SpawnMaze { floor, ..default() });
    }
}
