use crate::{
    maze::{
        components::{Floor, Maze},
        MazeConfig,
    },
    player::components::{CurrentPosition, MovementTarget, Player},
};
use bevy::prelude::*;
use hexx::{EdgeDirection, HexOrientation};

fn create_direction(
    input: &ButtonInput<KeyCode>,
    orientation: &HexOrientation,
) -> Option<EdgeDirection> {
    let w = input.pressed(KeyCode::KeyW);
    let a = input.pressed(KeyCode::KeyA);
    let s = input.pressed(KeyCode::KeyS);
    let d = input.pressed(KeyCode::KeyD);

    let direction = match orientation {
        HexOrientation::Pointy => {
            match (w, a, s, d) {
                (true, false, false, false) => Some(EdgeDirection::POINTY_WEST), // W
                (false, false, true, false) => Some(EdgeDirection::POINTY_EAST), // S
                (false, true, true, false) => Some(EdgeDirection::POINTY_NORTH_EAST), // A+S
                (false, false, true, true) => Some(EdgeDirection::POINTY_SOUTH_EAST), // S+D
                (true, true, false, false) => Some(EdgeDirection::POINTY_NORTH_WEST), // W+A
                (true, false, false, true) => Some(EdgeDirection::POINTY_SOUTH_WEST), // W+D
                _ => None,
            }
        }
        HexOrientation::Flat => {
            match (w, a, s, d) {
                (false, true, false, false) => Some(EdgeDirection::FLAT_NORTH), // A
                (false, false, false, true) => Some(EdgeDirection::FLAT_SOUTH), // D
                (false, true, true, false) => Some(EdgeDirection::FLAT_NORTH_EAST), // A+S
                (false, false, true, true) => Some(EdgeDirection::FLAT_SOUTH_EAST), // S+D
                (true, true, false, false) => Some(EdgeDirection::FLAT_NORTH_WEST), // W+A
                (true, false, false, true) => Some(EdgeDirection::FLAT_SOUTH_WEST), // W+D
                _ => None,
            }
        }
    }?;
    Some(direction.rotate_cw(0))
}

pub(super) fn player_input(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut MovementTarget, &CurrentPosition), With<Player>>,
    maze_query: Query<(&Maze, &Floor)>,
    maze_config: Res<MazeConfig>,
) {
    let Ok((maze, _floor)) = maze_query.get_single() else {
        return;
    };

    for (mut target_pos, current_pos) in player_query.iter_mut() {
        if target_pos.is_some() {
            continue;
        }

        let Some(direction) = create_direction(&input, &maze_config.layout.orientation) else {
            continue;
        };

        let Some(tile) = maze.0.get_tile(&current_pos) else {
            continue;
        };

        if tile.walls().contains(direction) {
            continue;
        }

        let next_hex = current_pos.0.neighbor(direction);
        target_pos.0 = Some(next_hex);
    }
}
