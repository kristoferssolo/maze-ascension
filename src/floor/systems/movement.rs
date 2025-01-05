use crate::{
    constants::{FLOOR_Y_OFFSET, MOVEMENT_THRESHOLD},
    floor::{
        components::{CurrentFloor, Floor, FloorYTarget},
        events::TransitionFloor,
    },
    maze::components::HexMaze,
    player::components::{MovementSpeed, Player},
};

use bevy::prelude::*;

pub fn move_floors(
    mut commands: Commands,
    mut maze_query: Query<(Entity, &mut Transform, &FloorYTarget), With<FloorYTarget>>,
    player_query: Query<&MovementSpeed, With<Player>>,
    time: Res<Time>,
) {
    let speed = player_query.get_single().map_or(100., |s| s.0);
    let movement_distance = speed * time.delta_secs();
    for (entity, mut transform, movement_state) in maze_query.iter_mut() {
        let delta = movement_state.0 - transform.translation.y;
        if delta.abs() > MOVEMENT_THRESHOLD {
            let movement = delta.signum() * movement_distance.min(delta.abs());
            transform.translation.y += movement;
        } else {
            transform.translation.y = movement_state.0;
            commands.entity(entity).remove::<FloorYTarget>();
        }
    }
}

pub fn handle_floor_transition_events(
    mut commands: Commands,
    mut maze_query: Query<(Entity, &Transform, &Floor, Option<&FloorYTarget>), With<HexMaze>>,
    current_query: Query<(Entity, &Floor), With<CurrentFloor>>,
    mut event_reader: EventReader<TransitionFloor>,
) {
    let is_moving = maze_query
        .iter()
        .any(|(_, _, _, movement_state)| movement_state.is_some());

    if is_moving {
        return;
    }

    for event in event_reader.read() {
        let Some((current_entity, current_floor)) = current_query.get_single().ok() else {
            continue;
        };

        let target_floor_num = event.next_floor_num(current_floor);

        let target_entity = maze_query
            .iter()
            .find(|(_, _, floor, _)| floor.0 == target_floor_num)
            .map(|(entity, ..)| entity);

        let Some(target_entity) = target_entity else {
            continue;
        };

        let direction = event.into();

        for (entity, transforms, _, movement_state) in maze_query.iter_mut() {
            let target_y = (FLOOR_Y_OFFSET as f32).mul_add(direction, transforms.translation.y);
            if movement_state.is_none() {
                commands.entity(entity).insert(FloorYTarget(target_y));
            }
        }

        update_current_next_floor(&mut commands, current_entity, target_entity);
        break;
    }
}

fn update_current_next_floor(commands: &mut Commands, current: Entity, target: Entity) {
    commands.entity(current).remove::<CurrentFloor>();
    commands.entity(target).insert(CurrentFloor);
}
