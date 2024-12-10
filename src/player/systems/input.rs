use crate::player::components::Player;
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

pub(super) fn player_input(input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Player>) {
    for mut player in player_query.iter_mut() {
        if player.target_hex.is_some() {
            continue;
        }

        if let Some(direction) = input.get_pressed().find_map(|key| create_direction(key)) {
            let next_hex = player.current_hex + direction.into_hex();
            player.target_hex = Some(next_hex);
        }
    }
}
