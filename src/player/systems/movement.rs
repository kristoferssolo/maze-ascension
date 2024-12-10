use crate::{maze::MazeConfig, player::components::Player};
use bevy::prelude::*;
use hexx::Hex;

pub(super) fn player_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    maze_config: Res<MazeConfig>,
) {
    for (mut player, mut transform) in player_query.iter_mut() {
        if let Some(target_hex) = player.target_hex {
            let current_pos = transform.translation;
            let target_pos = calculate_target_position(&maze_config, target_hex, current_pos.y);

            if should_complete_movement(current_pos, target_pos) {
                complete_movement(&mut player, &mut transform, target_pos, target_hex);
                continue;
            }

            update_position(
                &mut transform,
                current_pos,
                target_pos,
                player.speed,
                time.delta_seconds(),
            );
        }
    }
}

fn calculate_target_position(maze_config: &MazeConfig, target_hex: Hex, y: f32) -> Vec3 {
    let world_pos = maze_config.layout.hex_to_world_pos(target_hex);
    Vec3::new(world_pos.x, y, world_pos.y)
}

fn should_complete_movement(current_pos: Vec3, target_pos: Vec3) -> bool {
    (target_pos - current_pos).length() < 0.1
}

fn complete_movement(
    player: &mut Player,
    transform: &mut Transform,
    target_pos: Vec3,
    target_hex: Hex,
) {
    transform.translation = target_pos;
    player.current_hex = target_hex;
    player.target_hex = None;
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
