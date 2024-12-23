use crate::{
    floor::components::CurrentFloor,
    maze::components::MazeConfig,
    player::components::{CurrentPosition, MovementSpeed, MovementTarget, Player},
};
use bevy::prelude::*;
use hexx::Hex;

const MOVEMENT_THRESHOLD: f32 = 0.1;

pub(super) fn player_movement(
    time: Res<Time>,
    mut query: Query<
        (
            &mut MovementTarget,
            &MovementSpeed,
            &mut CurrentPosition,
            &mut Transform,
        ),
        With<Player>,
    >,
    maze_config_query: Query<&MazeConfig, With<CurrentFloor>>,
) {
    let Ok(maze_config) = maze_config_query.get_single() else {
        warn!("Failed to get maze configuration for current floor - cannot move player");
        return;
    };

    for (mut target, speed, mut current_hex, mut transform) in query.iter_mut() {
        if let Some(target_hex) = target.0 {
            let current_pos = transform.translation;
            let target_pos = calculate_target_position(maze_config, target_hex, current_pos.y);

            if should_complete_movement(current_pos, target_pos) {
                transform.translation = target_pos;
                current_hex.0 = target_hex;
                target.0 = None;
                continue;
            }

            update_position(
                &mut transform,
                current_pos,
                target_pos,
                speed.0,
                time.delta_secs(),
            );
        }
    }
}

fn should_complete_movement(current_pos: Vec3, target_pos: Vec3) -> bool {
    (target_pos - current_pos).length() < MOVEMENT_THRESHOLD
}

fn update_position(
    transform: &mut Transform,
    current_pos: Vec3,
    target_pos: Vec3,
    speed: f32,
    delta_time: f32,
) {
    let direction = target_pos - current_pos;
    let movement = direction.normalize() * speed * delta_time;

    if movement.length() > direction.length() {
        transform.translation = target_pos;
        return;
    }
    transform.translation += movement;
}

fn calculate_target_position(maze_config: &MazeConfig, target_hex: Hex, y: f32) -> Vec3 {
    let world_pos = maze_config.layout.hex_to_world_pos(target_hex);
    Vec3::new(world_pos.x, y, world_pos.y)
}
