use crate::{
    floor::{
        components::{CurrentFloor, NextFloor},
        events::TransitionFloor,
    },
    maze::{components::Maze, GlobalMazeConfig},
    player::components::{MovementSpeed, Player},
};
use bevy::prelude::*;

pub(super) fn floor_movement(
    mut commands: Commands,
    mut maze_query: Query<(Entity, &mut Transform), With<Maze>>,
    current_query: Query<Entity, With<CurrentFloor>>,
    next_query: Query<Entity, With<NextFloor>>,
    player_query: Query<&MovementSpeed, With<Player>>,
    time: Res<Time>,
    global_config: Res<GlobalMazeConfig>,
    mut event_reader: EventReader<TransitionFloor>,
) {
    let speed = player_query.get_single().map(|s| s.0).unwrap_or(100.);
    let movement_distance = speed * time.delta_secs();

    for event in event_reader.read() {
        let y_offset = match event {
            TransitionFloor::Ascend => -global_config.height,
            TransitionFloor::Descend => global_config.height,
        };

        for (_, mut transforms) in maze_query.iter_mut() {
            let target_y = transforms.translation.y + y_offset;
            let delta = target_y - transforms.translation.y;
            if delta.abs() > 0.001 {
                let movement = delta.signum() * movement_distance.min(delta.abs());
                transforms.translation.y += movement;
            } else {
                transforms.translation.y = target_y;
            }
        }

        // Update current/next floor
        if let (Ok(current_entity), Ok(next_entity)) =
            (current_query.get_single(), next_query.get_single())
        {
            commands.entity(current_entity).remove::<CurrentFloor>();
            commands
                .entity(next_entity)
                .remove::<NextFloor>()
                .insert(CurrentFloor);
        }
    }
}
