use crate::{maze::MazeConfig, player::components::Player};
use bevy::prelude::*;
use hexx::EdgeDirection;

const fn create_direction(key: &KeyCode) -> Option<EdgeDirection> {
    match key {
        KeyCode::KeyD => Some(EdgeDirection::FLAT_SOUTH),
        KeyCode::KeyS => Some(EdgeDirection::FLAT_NORTH_EAST),
        KeyCode::KeyA => Some(EdgeDirection::FLAT_NORTH),
        KeyCode::KeyQ => Some(EdgeDirection::FLAT_NORTH_WEST),
        KeyCode::KeyW => Some(EdgeDirection::FLAT_SOUTH_WEST),
        KeyCode::KeyE => Some(EdgeDirection::FLAT_SOUTH_EAST),
        _ => None,
    }
}

pub fn player_movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    maze_config: Res<MazeConfig>,
) {
    for (mut player, mut transform) in player_query.iter_mut() {
        if let Some(direction) = input.get_pressed().find_map(|key| create_direction(key)) {
            let next_hex = player.current_hex + direction.into_hex();
            player.target_hex = Some(next_hex);
        }

        if let Some(target_hex) = player.target_hex {
            let current_pos = transform.translation;
            let target_pos = {
                let world_pos = maze_config.layout.hex_to_world_pos(target_hex);
                Vec3::new(world_pos.x, current_pos.y, world_pos.y)
            };
            let direction = target_pos - current_pos;
            let distance = direction.length();

            if distance < 0.1 {
                transform.translation = target_pos;
                player.current_hex = target_hex;
                player.target_hex = None;
            } else {
                let movement = direction.normalize() * player.speed * time.delta_seconds();
                if movement.length() > distance {
                    transform.translation = target_pos;
                } else {
                    transform.translation += movement;
                }
            }
        }
    }
}
