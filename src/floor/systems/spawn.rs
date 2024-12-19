use bevy::prelude::*;

use crate::{
    floor::{
        components::{CurrentFloor, Floor},
        events::TransitionFloor,
    },
    maze::events::SpawnMaze,
};

pub(super) fn spawn_floor(
    mut commands: Commands,
    query: Query<(Entity, &Floor), With<CurrentFloor>>,
    mut event_reader: EventReader<TransitionFloor>,
) {
    let Ok((entity, floor)) = query.get_single() else {
        return;
    };

    for event in event_reader.read() {
        dbg!(event);
        let floor = match event {
            TransitionFloor::Ascend => *floor.increase(),
            TransitionFloor::Descent => *floor.decrease(),
        };

        if floor == 1 {
            return;
        }

        info!("Creating level for floor {}", floor);

        commands.entity(entity).remove::<CurrentFloor>();
        commands.trigger(SpawnMaze { floor, ..default() });
    }
}
