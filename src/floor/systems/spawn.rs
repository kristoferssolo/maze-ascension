use crate::{
    floor::{
        components::{CurrentFloor, Floor, FloorYTarget},
        events::TransitionFloor,
        resources::HighestFloor,
    },
    maze::{components::MazeConfig, events::SpawnMaze},
};
use bevy::prelude::*;

pub(super) fn spawn_floor(
    mut commands: Commands,
    query: Query<(&mut Floor, &MazeConfig), (With<CurrentFloor>, Without<FloorYTarget>)>,
    mut event_reader: EventReader<TransitionFloor>,
    mut highest_floor: ResMut<HighestFloor>,
) {
    let Ok((current_floor, config)) = query.get_single() else {
        return;
    };

    for event in event_reader.read() {
        if current_floor.0 == 0 && *event == TransitionFloor::Descend {
            warn!("Cannot descend below floor 1");
            return;
        }

        let target_floor = event.next_floor_num(current_floor);
        highest_floor.0 = highest_floor.0.max(target_floor);

        info!("Creating level for floor {}", target_floor);

        commands.trigger(SpawnMaze {
            floor: target_floor,
            config: MazeConfig {
                start_pos: config.end_pos,
                radius: config.radius + 1,
                ..default()
            },
        });
    }
}
