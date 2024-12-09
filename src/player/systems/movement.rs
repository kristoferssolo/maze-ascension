use bevy::prelude::*;
use hexx::{EdgeDirection, Hex};

use crate::{maze::MazeConfig, player::components::Player};

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
    mut player_query: Query<(&Player, &mut Transform)>,
    maze_config: Res<MazeConfig>,
) {
    for (player, mut transform) in player_query.iter_mut() {
        let direction = input.get_pressed().find_map(|key| create_direction(key));

        if let Some(hex_dir) = direction {
            let hex_vec = Hex::from(hex_dir);
            let world_pos = maze_config.layout.hex_to_world_pos(hex_vec);

            let move_vec = Vec3::new(world_pos.x, 0.0, world_pos.y).normalize()
                * player.speed
                * time.delta_seconds();

            transform.translation += move_vec;
        }
    }
}
