use crate::{
    floor::{
        components::{CurrentFloor, Floor, FloorYTarget},
        events::TransitionFloor,
        resources::HighestFloor,
    },
    maze::events::SpawnMaze,
};
use bevy::prelude::*;

pub(super) fn spawn_floor(
    mut commands: Commands,
    query: Query<&mut Floor, (With<CurrentFloor>, Without<FloorYTarget>)>,
    mut event_reader: EventReader<TransitionFloor>,
    mut highest_floor: ResMut<HighestFloor>,
) {
    let Ok(floor) = query.get_single() else {
        return;
    };

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
