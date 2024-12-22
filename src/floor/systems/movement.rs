use bevy::prelude::*;

use crate::{
    floor::{
        components::{CurrentFloor, NextFloor},
        events::TransitionFloor,
    },
    maze::{components::Maze, GlobalMazeConfig},
    player::components::{MovementSpeed, Player},
};

pub(super) fn floor_movement(
    mut commands: Commands,
    mut maze_transforms: Query<(Entity, &mut Transform), With<Maze>>,
    current_floor: Query<Entity, With<CurrentFloor>>,
    next_floor: Query<Entity, With<NextFloor>>,
    player_query: Query<&MovementSpeed, With<Player>>,
    time: Res<Time>,
    global_config: Res<GlobalMazeConfig>,
    mut event_reader: EventReader<TransitionFloor>,
) {
    let speed = player_query.get_single().map(|s| s.0).unwrap_or(100.);
    let movement_distance = speed * time.delta_secs();

    for event in event_reader.read() {
        let (direction, target_y) = match event {
            TransitionFloor::Ascend => (Vec3::Y, -global_config.height),
            TransitionFloor::Descent => (Vec3::NEG_Y, global_config.height),
        };

        let movement = direction * movement_distance;

        for (_, mut transform) in maze_transforms.iter_mut() {
            transform.translation += movement;
        }

        let is_movement_complete = maze_transforms
            .iter()
            .any(|(_, t)| t.translation.y.abs() >= target_y.abs());

        if is_movement_complete {
            if let Ok(current_floor_entity) = current_floor.get_single() {
                commands
                    .entity(current_floor_entity)
                    .remove::<CurrentFloor>();
            }

            if let Ok(next_floor_entity) = next_floor.get_single() {
                if let Ok((entity, mut transform)) = maze_transforms.get_mut(next_floor_entity) {
                    transform.translation.y = 0.;
                    commands
                        .entity(entity)
                        .remove::<NextFloor>()
                        .insert(CurrentFloor);
                }
            }
        }
    }
}
