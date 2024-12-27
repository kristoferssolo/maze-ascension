use crate::{
    floor::{
        components::{CurrentFloor, Floor, MovementState},
        events::TransitionFloor,
        resources::HighestFloor,
    },
    maze::events::SpawnMaze,
};
use bevy::prelude::*;

pub(super) fn spawn_floor(
    mut commands: Commands,
    query: Query<&mut Floor, With<CurrentFloor>>,
    movement_state_query: Query<Option<&MovementState>>,
    mut event_reader: EventReader<TransitionFloor>,
    mut highest_floor: ResMut<HighestFloor>,
) {
    let Ok(floor) = query.get_single() else {
        return;
    };

    let is_moving = movement_state_query
        .iter()
        .any(|movement_state| movement_state.is_some());
    if is_moving {
        return;
    }

    for event in event_reader.read() {
        let floor = event.next_floor_num(floor);

        if floor == 1 && *event == TransitionFloor::Descend {
            warn!("Cannot descend below floor 1");
            return;
        }

        highest_floor.0 = highest_floor.0.max(floor);

        info!("Creating level for floor {}", floor);

        commands.trigger(SpawnMaze { floor, ..default() });
    }
}
