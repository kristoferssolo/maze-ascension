use crate::{
    floor::{
        components::{CurrentFloor, MovementState, NextFloor},
        events::TransitionFloor,
    },
    maze::{components::Maze, GlobalMazeConfig},
    player::components::{MovementSpeed, Player},
};
use bevy::prelude::*;

const MOVEMENT_THRESHOLD: f32 = 0.001;

pub(super) fn floor_movement(
    mut commands: Commands,
    mut maze_query: Query<(Entity, &mut Transform, Option<&mut MovementState>), With<Maze>>,
    current_query: Query<Entity, With<CurrentFloor>>,
    next_query: Query<Entity, With<NextFloor>>,
    player_query: Query<&MovementSpeed, With<Player>>,
    time: Res<Time>,
    _global_config: Res<GlobalMazeConfig>,
    mut event_reader: EventReader<TransitionFloor>,
) {
    for event in event_reader.read() {
        let direction = match event {
            TransitionFloor::Ascend => -1.,
            TransitionFloor::Descend => 1.,
        };

        let Some((current_entity, current_y)) = get_floor_info(&maze_query, &current_query) else {
            continue;
        };
        let Some((next_entity, next_y)) = get_floor_info(&maze_query, &next_query) else {
            continue;
        };
        let diff = (current_y - next_y).abs();

        for (entity, transforms, movement_state) in maze_query.iter_mut() {
            let target_y = diff.mul_add(direction, transforms.translation.y);
            if movement_state.is_none() {
                commands.entity(entity).insert(MovementState {
                    target_y,
                    direction,
                });
            }
        }

        update_current_next_floor(&mut commands, current_entity, next_entity);
    }

    let speed = player_query.get_single().map_or(100., |s| s.0);
    let movement_distance = speed * time.delta_secs();
    for (entity, mut transform, mut movement_state) in maze_query.iter_mut() {
        if let Some(state) = movement_state.as_mut() {
            let delta = state.target_y - transform.translation.y;
            if delta.abs() > MOVEMENT_THRESHOLD {
                let movement = delta.signum() * movement_distance.min(delta.abs());
                transform.translation.y += movement;
            } else {
                transform.translation.y = state.target_y;
                commands.entity(entity).remove::<MovementState>();
            }
        }
    }
}

fn update_current_next_floor(commands: &mut Commands, current_entity: Entity, next_entity: Entity) {
    commands.entity(current_entity).remove::<CurrentFloor>();
    commands
        .entity(next_entity)
        .remove::<NextFloor>()
        .insert(CurrentFloor);
}

fn get_floor_info(
    maze_query: &Query<(Entity, &mut Transform, Option<&mut MovementState>), With<Maze>>,
    query: &Query<Entity, With<impl Component>>,
) -> Option<(Entity, f32)> {
    query.get_single().ok().and_then(|entity| {
        maze_query
            .get(entity)
            .ok()
            .map(|(_, transform, _)| (entity, transform.translation.y))
    })
}
