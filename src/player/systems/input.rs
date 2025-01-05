use crate::{
    floor::components::{CurrentFloor, FloorYTarget},
    maze::components::MazeConfig,
    player::components::{CurrentPosition, MovementTarget, Player},
};
use bevy::prelude::*;
use hexlab::prelude::*;
use hexx::{EdgeDirection, HexOrientation};

/// Handles player movement input based on keyboard controls and maze configuration
pub fn player_input(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut MovementTarget, &CurrentPosition), With<Player>>,
    maze_query: Query<(&Maze, &MazeConfig, Has<FloorYTarget>), With<CurrentFloor>>,
) {
    let Ok((maze, maze_config, has_y_target)) = maze_query.get_single() else {
        return;
    };

    // Disable movement while transitioning floors
    if has_y_target {
        return;
    }

    for (mut target_pos, current_pos) in player_query.iter_mut() {
        if target_pos.is_some() {
            continue;
        }

        let Some(tile) = maze.get(current_pos) else {
            continue;
        };

        let Ok(key_direction) = KeyDirection::try_from(&*input) else {
            continue;
        };

        let possible_directions = key_direction.related_directions(&maze_config.layout.orientation);

        // Convert to edge directions and filter out walls
        let mut available_directions = possible_directions
            .into_iter()
            .map(EdgeDirection::from)
            .filter(|dir| !tile.walls().contains(*dir))
            .collect::<Vec<_>>();

        if let Some(logical_dir) = key_direction.exact_direction(&maze_config.layout.orientation) {
            let edge_dir = EdgeDirection::from(logical_dir);
            if available_directions.contains(&edge_dir) {
                available_directions = vec![edge_dir];
            }
        }

        if available_directions.len() == 1 {
            if let Some(&next_tile) = available_directions.first() {
                let next_hex = current_pos.0.neighbor(next_tile);
                target_pos.0 = Some(next_hex);
            }
        }
    }
}

/// Represents possible movement directions from keyboard input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyDirection {
    Up,        // Single press: W
    Right,     // Single press: D
    Down,      // Single press: S
    Left,      // Single press: A
    UpRight,   // Diagonal: W+D
    UpLeft,    // Diagonal: W+A
    DownRight, // Diagonal: S+D
    DownLeft,  // Diagonal: S+A
}

impl KeyDirection {
    /// Converts key direction to exact logical direction based on hex orientation
    fn exact_direction(&self, orientation: &HexOrientation) -> Option<LogicalDirection> {
        match orientation {
            HexOrientation::Pointy => match self {
                KeyDirection::Up => Some(LogicalDirection::PointyNorth),
                KeyDirection::Down => Some(LogicalDirection::PointySouth),
                KeyDirection::UpRight => Some(LogicalDirection::PointyNorthEast),
                KeyDirection::UpLeft => Some(LogicalDirection::PointyNorthWest),
                KeyDirection::DownRight => Some(LogicalDirection::PointySouthEast),
                KeyDirection::DownLeft => Some(LogicalDirection::PointySouthWest),
                _ => None,
            },
            HexOrientation::Flat => match self {
                KeyDirection::Right => Some(LogicalDirection::FlatEast),
                KeyDirection::Left => Some(LogicalDirection::FlatWest),
                KeyDirection::UpRight => Some(LogicalDirection::FlatNorthEast),
                KeyDirection::UpLeft => Some(LogicalDirection::FlatNorthWest),
                KeyDirection::DownRight => Some(LogicalDirection::FlatSouthEast),
                KeyDirection::DownLeft => Some(LogicalDirection::FlatSouthWest),
                _ => None,
            },
        }
    }

    /// Returns all possible logical directions for the given key input and hex orientation
    fn related_directions(&self, orientation: &HexOrientation) -> Vec<LogicalDirection> {
        match orientation {
            HexOrientation::Pointy => match self {
                // Single key presses check multiple directions
                KeyDirection::Up => vec![
                    LogicalDirection::PointyNorth,
                    LogicalDirection::PointyNorthEast,
                    LogicalDirection::PointyNorthWest,
                ],
                KeyDirection::Right => vec![
                    LogicalDirection::PointyNorthEast,
                    LogicalDirection::PointySouthEast,
                ],
                KeyDirection::Down => vec![
                    LogicalDirection::PointySouth,
                    LogicalDirection::PointySouthEast,
                    LogicalDirection::PointySouthWest,
                ],
                KeyDirection::Left => vec![
                    LogicalDirection::PointyNorthWest,
                    LogicalDirection::PointySouthWest,
                ],
                // Diagonal combinations check specific directions
                KeyDirection::UpRight => vec![LogicalDirection::PointyNorthEast],
                KeyDirection::UpLeft => vec![LogicalDirection::PointyNorthWest],
                KeyDirection::DownRight => vec![LogicalDirection::PointySouthEast],
                KeyDirection::DownLeft => vec![LogicalDirection::PointySouthWest],
            },
            HexOrientation::Flat => match self {
                KeyDirection::Up => vec![
                    LogicalDirection::FlatNorthEast,
                    LogicalDirection::FlatNorthWest,
                ],
                KeyDirection::Right => vec![
                    LogicalDirection::FlatEast,
                    LogicalDirection::FlatNorthEast,
                    LogicalDirection::FlatSouthEast,
                ],
                KeyDirection::Down => vec![
                    LogicalDirection::FlatSouthEast,
                    LogicalDirection::FlatSouthWest,
                ],
                KeyDirection::Left => vec![
                    LogicalDirection::FlatWest,
                    LogicalDirection::FlatNorthWest,
                    LogicalDirection::FlatSouthWest,
                ],
                // Diagonal combinations check specific directions
                KeyDirection::UpRight => vec![LogicalDirection::FlatNorthEast],
                KeyDirection::UpLeft => vec![LogicalDirection::FlatNorthWest],
                KeyDirection::DownRight => vec![LogicalDirection::FlatSouthEast],
                KeyDirection::DownLeft => vec![LogicalDirection::FlatSouthWest],
            },
        }
    }
}

impl TryFrom<&ButtonInput<KeyCode>> for KeyDirection {
    type Error = String;
    fn try_from(value: &ButtonInput<KeyCode>) -> Result<Self, Self::Error> {
        let w = value.pressed(KeyCode::KeyW);
        let a = value.pressed(KeyCode::KeyA);
        let s = value.pressed(KeyCode::KeyS);
        let d = value.pressed(KeyCode::KeyD);

        match (w, a, s, d) {
            // Single key presses
            (true, false, false, false) => Ok(Self::Up),
            (false, true, false, false) => Ok(Self::Left),
            (false, false, true, false) => Ok(Self::Down),
            (false, false, false, true) => Ok(Self::Right),
            // Diagonal combinations
            (true, false, false, true) => Ok(Self::UpRight),
            (true, true, false, false) => Ok(Self::UpLeft),
            (false, false, true, true) => Ok(Self::DownRight),
            (false, true, true, false) => Ok(Self::DownLeft),
            _ => Err("Invalid direction key combination".to_owned()),
        }
    }
}

/// Represents logical directions in both pointy and flat hex orientations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogicalDirection {
    // For Pointy orientation
    PointyNorth,     // W
    PointySouth,     // S
    PointyNorthEast, // W+D
    PointySouthEast, // S+D
    PointyNorthWest, // W+A
    PointySouthWest, // S+A

    // For Flat orientation
    FlatWest,      // A
    FlatEast,      // D
    FlatNorthEast, // W+D
    FlatSouthEast, // S+D
    FlatNorthWest, // W+A
    FlatSouthWest, // S+A
}

impl From<LogicalDirection> for EdgeDirection {
    fn from(value: LogicalDirection) -> Self {
        let direction = match value {
            // Pointy orientation mappings
            LogicalDirection::PointyNorth => Self::POINTY_WEST,
            LogicalDirection::PointySouth => Self::POINTY_EAST,
            LogicalDirection::PointyNorthEast => Self::POINTY_SOUTH_WEST,
            LogicalDirection::PointySouthEast => Self::POINTY_SOUTH_EAST,
            LogicalDirection::PointyNorthWest => Self::POINTY_NORTH_WEST,
            LogicalDirection::PointySouthWest => Self::POINTY_NORTH_EAST,

            // Flat orientation mappings
            LogicalDirection::FlatWest => Self::FLAT_NORTH,
            LogicalDirection::FlatEast => Self::FLAT_SOUTH,
            LogicalDirection::FlatNorthEast => Self::FLAT_SOUTH_WEST,
            LogicalDirection::FlatSouthEast => Self::FLAT_SOUTH_EAST,
            LogicalDirection::FlatNorthWest => Self::FLAT_NORTH_WEST,
            LogicalDirection::FlatSouthWest => Self::FLAT_NORTH_EAST,
        };
        direction.rotate_cw(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a button input with specific key states
    fn create_input(w: bool, a: bool, s: bool, d: bool) -> ButtonInput<KeyCode> {
        let mut input = ButtonInput::default();
        if w {
            input.press(KeyCode::KeyW);
        }
        if a {
            input.press(KeyCode::KeyA);
        }
        if s {
            input.press(KeyCode::KeyS);
        }
        if d {
            input.press(KeyCode::KeyD);
        }
        input
    }

    #[test]
    fn key_direction_single_keys() {
        assert!(matches!(
            KeyDirection::try_from(&create_input(true, false, false, false)),
            Ok(KeyDirection::Up)
        ));
        assert!(matches!(
            KeyDirection::try_from(&create_input(false, true, false, false)),
            Ok(KeyDirection::Left)
        ));
        assert!(matches!(
            KeyDirection::try_from(&create_input(false, false, true, false)),
            Ok(KeyDirection::Down)
        ));
        assert!(matches!(
            KeyDirection::try_from(&create_input(false, false, false, true)),
            Ok(KeyDirection::Right)
        ));
    }

    #[test]
    fn key_direction_diagonal_combinations() {
        assert!(matches!(
            KeyDirection::try_from(&create_input(true, false, false, true)),
            Ok(KeyDirection::UpRight)
        ));
        assert!(matches!(
            KeyDirection::try_from(&create_input(true, true, false, false)),
            Ok(KeyDirection::UpLeft)
        ));
        assert!(matches!(
            KeyDirection::try_from(&create_input(false, false, true, true)),
            Ok(KeyDirection::DownRight)
        ));
        assert!(matches!(
            KeyDirection::try_from(&create_input(false, true, true, false)),
            Ok(KeyDirection::DownLeft)
        ));
    }

    #[test]
    fn key_direction_invalid_combinations() {
        assert!(KeyDirection::try_from(&create_input(true, true, true, false)).is_err());
        assert!(KeyDirection::try_from(&create_input(true, true, false, true)).is_err());
        assert!(KeyDirection::try_from(&create_input(true, true, true, true)).is_err());
    }

    #[test]
    fn exact_direction_pointy() {
        let orientation = HexOrientation::Pointy;

        assert_eq!(
            KeyDirection::Up.exact_direction(&orientation),
            Some(LogicalDirection::PointyNorth)
        );
        assert_eq!(
            KeyDirection::Down.exact_direction(&orientation),
            Some(LogicalDirection::PointySouth)
        );
        assert_eq!(
            KeyDirection::UpRight.exact_direction(&orientation),
            Some(LogicalDirection::PointyNorthEast)
        );
    }

    #[test]
    fn exact_direction_flat() {
        let orientation = HexOrientation::Flat;

        assert_eq!(
            KeyDirection::Right.exact_direction(&orientation),
            Some(LogicalDirection::FlatEast)
        );
        assert_eq!(
            KeyDirection::Left.exact_direction(&orientation),
            Some(LogicalDirection::FlatWest)
        );
        assert_eq!(
            KeyDirection::UpRight.exact_direction(&orientation),
            Some(LogicalDirection::FlatNorthEast)
        );
    }

    #[test]
    fn related_directions_pointy() {
        let orientation = HexOrientation::Pointy;

        let up_directions = KeyDirection::Up.related_directions(&orientation);
        assert!(up_directions.contains(&LogicalDirection::PointyNorth));
        assert!(up_directions.contains(&LogicalDirection::PointyNorthEast));
        assert!(up_directions.contains(&LogicalDirection::PointyNorthWest));
    }

    #[test]
    fn related_directions_flat() {
        let orientation = HexOrientation::Flat;

        let right_directions = KeyDirection::Right.related_directions(&orientation);
        assert!(right_directions.contains(&LogicalDirection::FlatEast));
        assert!(right_directions.contains(&LogicalDirection::FlatNorthEast));
        assert!(right_directions.contains(&LogicalDirection::FlatSouthEast));
    }
}
