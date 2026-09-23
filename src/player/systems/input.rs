use crate::{
    floor::components::{CurrentFloor, FloorYTarget},
    maze::components::MazeConfig,
    player::components::{CurrentPosition, MovementTarget, Player},
    powerups::wall_jump::WallJump,
};
use bevy::prelude::*;
use hexlab::prelude::*;
use hexx::{EdgeDirection, HexOrientation};

/// Handles player movement input based on keyboard controls and maze configuration
pub fn player_input(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut MovementTarget, &CurrentPosition), With<Player>>,
    maze_query: Query<(&Maze, &MazeConfig, Option<&FloorYTarget>), With<CurrentFloor>>,
    mut wall_jump: Option<ResMut<WallJump>>,
) {
    let Ok((maze, maze_config, y_target)) = maze_query.single() else {
        return;
    };

    // Disable movement while transitioning floors
    if y_target.is_some() {
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

        let possible_directions = key_direction
            .related_directions(&maze_config.layout.orientation)
            .into_iter()
            .map(EdgeDirection::from)
            .collect::<Vec<_>>();

        if input.just_pressed(KeyCode::Space)
            && wall_jump.as_ref().is_some_and(|powerup| powerup.is_ready())
        {
            let jump_directions = possible_directions
                .iter()
                .copied()
                .filter(|direction| {
                    maze.get(&current_pos.0.neighbor(*direction))
                        .is_some_and(|neighbor| {
                            tile.walls().contains(*direction)
                                || neighbor.walls().contains(-*direction)
                        })
                })
                .collect::<Vec<_>>();

            if let Some(direction) = selected_direction(
                key_direction,
                &maze_config.layout.orientation,
                &jump_directions,
            ) {
                target_pos.0 = Some(current_pos.0.neighbor(direction));
                if let Some(powerup) = &mut wall_jump {
                    powerup.consume();
                }
                continue;
            }
        }

        // Convert to edge directions and filter out walls
        let available_directions = possible_directions
            .iter()
            .copied()
            .filter(|dir| {
                !tile.walls().contains(*dir)
                    && maze
                        .get(&current_pos.0.neighbor(*dir))
                        .is_some_and(|neighbor| !neighbor.walls().contains(-*dir))
            })
            .collect::<Vec<_>>();

        if let Some(direction) = selected_direction(
            key_direction,
            &maze_config.layout.orientation,
            &available_directions,
        ) {
            target_pos.0 = Some(current_pos.0.neighbor(direction));
        }
    }
}

fn selected_direction(
    key_direction: KeyDirection,
    orientation: &HexOrientation,
    candidates: &[EdgeDirection],
) -> Option<EdgeDirection> {
    if let Some(exact) = key_direction
        .exact_direction(orientation)
        .map(EdgeDirection::from)
    {
        if candidates.contains(&exact) {
            return Some(exact);
        }
    }
    match candidates {
        [direction] => Some(*direction),
        _ => None,
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
    const fn exact_direction(&self, orientation: &HexOrientation) -> Option<LogicalDirection> {
        match orientation {
            HexOrientation::Pointy => match self {
                Self::Up => Some(LogicalDirection::PointyNorth),
                Self::Down => Some(LogicalDirection::PointySouth),
                Self::UpRight => Some(LogicalDirection::PointyNorthEast),
                Self::UpLeft => Some(LogicalDirection::PointyNorthWest),
                Self::DownRight => Some(LogicalDirection::PointySouthEast),
                Self::DownLeft => Some(LogicalDirection::PointySouthWest),
                _ => None,
            },
            HexOrientation::Flat => match self {
                Self::Right => Some(LogicalDirection::FlatEast),
                Self::Left => Some(LogicalDirection::FlatWest),
                Self::UpRight => Some(LogicalDirection::FlatNorthEast),
                Self::UpLeft => Some(LogicalDirection::FlatNorthWest),
                Self::DownRight => Some(LogicalDirection::FlatSouthEast),
                Self::DownLeft => Some(LogicalDirection::FlatSouthWest),
                _ => None,
            },
        }
    }

    /// Returns all possible logical directions for the given key input and hex orientation
    fn related_directions(&self, orientation: &HexOrientation) -> Vec<LogicalDirection> {
        match orientation {
            HexOrientation::Pointy => match self {
                // Single key presses check multiple directions
                Self::Up => vec![
                    LogicalDirection::PointyNorth,
                    LogicalDirection::PointyNorthEast,
                    LogicalDirection::PointyNorthWest,
                ],
                Self::Right => vec![
                    LogicalDirection::PointyNorthEast,
                    LogicalDirection::PointySouthEast,
                ],
                Self::Down => vec![
                    LogicalDirection::PointySouth,
                    LogicalDirection::PointySouthEast,
                    LogicalDirection::PointySouthWest,
                ],
                Self::Left => vec![
                    LogicalDirection::PointyNorthWest,
                    LogicalDirection::PointySouthWest,
                ],
                // Diagonal combinations check specific directions
                Self::UpRight => vec![LogicalDirection::PointyNorthEast],
                Self::UpLeft => vec![LogicalDirection::PointyNorthWest],
                Self::DownRight => vec![LogicalDirection::PointySouthEast],
                Self::DownLeft => vec![LogicalDirection::PointySouthWest],
            },
            HexOrientation::Flat => match self {
                Self::Up => vec![
                    LogicalDirection::FlatNorthEast,
                    LogicalDirection::FlatNorthWest,
                ],
                Self::Right => vec![
                    LogicalDirection::FlatEast,
                    LogicalDirection::FlatNorthEast,
                    LogicalDirection::FlatSouthEast,
                ],
                Self::Down => vec![
                    LogicalDirection::FlatSouthEast,
                    LogicalDirection::FlatSouthWest,
                ],
                Self::Left => vec![
                    LogicalDirection::FlatWest,
                    LogicalDirection::FlatNorthWest,
                    LogicalDirection::FlatSouthWest,
                ],
                // Diagonal combinations check specific directions
                Self::UpRight => vec![LogicalDirection::FlatNorthEast],
                Self::UpLeft => vec![LogicalDirection::FlatNorthWest],
                Self::DownRight => vec![LogicalDirection::FlatSouthEast],
                Self::DownLeft => vec![LogicalDirection::FlatSouthWest],
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
        // The camera looks along -X, so screen right is -Z and screen up is -X.
        match value {
            // Pointy orientation mappings
            LogicalDirection::PointyNorth => Self::POINTY_WEST,
            LogicalDirection::PointySouth => Self::POINTY_EAST,
            LogicalDirection::PointyNorthEast => Self::POINTY_NORTH_WEST,
            LogicalDirection::PointySouthEast => Self::POINTY_NORTH_EAST,
            LogicalDirection::PointyNorthWest => Self::POINTY_SOUTH_WEST,
            LogicalDirection::PointySouthWest => Self::POINTY_SOUTH_EAST,

            // Flat orientation mappings
            LogicalDirection::FlatWest => Self::FLAT_SOUTH,
            LogicalDirection::FlatEast => Self::FLAT_NORTH,
            LogicalDirection::FlatNorthEast => Self::FLAT_NORTH_WEST,
            LogicalDirection::FlatSouthEast => Self::FLAT_NORTH_EAST,
            LogicalDirection::FlatNorthWest => Self::FLAT_SOUTH_WEST,
            LogicalDirection::FlatSouthWest => Self::FLAT_SOUTH_EAST,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_some;

    fn wall_jump_app(maze: Maze, keys: &[KeyCode]) -> (App, Entity) {
        let mut app = App::new();
        let mut input = ButtonInput::default();
        for &key in keys {
            input.press(key);
        }
        app.insert_resource(input);
        app.insert_resource(WallJump::default());
        app.add_systems(Update, player_input);
        app.world_mut()
            .spawn((CurrentFloor, MazeConfig::default(), maze));
        let player = app
            .world_mut()
            .spawn((Player, CurrentPosition::default()))
            .id();
        (app, player)
    }

    #[test]
    fn wall_jump_crosses_one_wall_and_starts_cooldown() {
        let direction = EdgeDirection::FLAT_NORTH_WEST;
        let mut maze = Maze::new();
        maze.insert(hexx::Hex::ZERO);
        maze.insert(hexx::Hex::ZERO.neighbor(direction));
        let (mut app, player) =
            wall_jump_app(maze, &[KeyCode::KeyW, KeyCode::KeyD, KeyCode::Space]);

        app.update();

        assert_eq!(
            app.world()
                .get::<MovementTarget>(player)
                .map(|target| target.0),
            Some(Some(hexx::Hex::ZERO.neighbor(direction)))
        );
        assert!(!assert_some!(app.world().get_resource::<WallJump>()).is_ready());
    }

    #[test]
    fn wall_jump_does_not_cross_maze_boundary_or_spend_cooldown() {
        let mut maze = Maze::new();
        maze.insert(hexx::Hex::ZERO);
        let (mut app, player) =
            wall_jump_app(maze, &[KeyCode::KeyW, KeyCode::KeyD, KeyCode::Space]);

        app.update();

        assert_eq!(
            app.world()
                .get::<MovementTarget>(player)
                .map(|target| target.0),
            Some(None)
        );
        assert!(assert_some!(app.world().get_resource::<WallJump>()).is_ready());
    }

    #[test]
    fn wall_jump_cannot_cross_while_on_cooldown() {
        let direction = EdgeDirection::FLAT_NORTH_WEST;
        let mut maze = Maze::new();
        maze.insert(hexx::Hex::ZERO);
        maze.insert(hexx::Hex::ZERO.neighbor(direction));
        let (mut app, player) =
            wall_jump_app(maze, &[KeyCode::KeyW, KeyCode::KeyD, KeyCode::Space]);
        assert_some!(app.world_mut().get_resource_mut::<WallJump>()).consume();

        app.update();

        assert_eq!(
            app.world()
                .get::<MovementTarget>(player)
                .map(|target| target.0),
            Some(None)
        );
    }

    fn movement_target_for(
        keys: &[KeyCode],
        orientation: HexOrientation,
        open_direction: EdgeDirection,
        neighbor_direction: EdgeDirection,
    ) -> Option<hexx::Hex> {
        let mut app = App::new();
        let mut input = ButtonInput::default();
        for &key in keys {
            input.press(key);
        }
        app.insert_resource(input);
        app.add_systems(Update, player_input);

        let mut maze = Maze::new();
        maze.insert(hexx::Hex::ZERO);
        let neighbor = hexx::Hex::ZERO.neighbor(neighbor_direction);
        maze.insert(neighbor);
        assert!(maze
            .remove_tile_wall(&hexx::Hex::ZERO, open_direction)
            .is_ok());
        assert!(maze
            .remove_tile_wall(&neighbor, open_direction.const_neg())
            .is_ok());
        let mut config = MazeConfig::default();
        config.layout.orientation = orientation;
        app.world_mut().spawn((CurrentFloor, config, maze));
        let player = app
            .world_mut()
            .spawn((Player, CurrentPosition::default()))
            .id();

        app.update();
        app.world()
            .get::<MovementTarget>(player)
            .and_then(|target| target.0)
    }

    #[test]
    fn flat_up_right_uses_screen_north_east_edge() {
        assert_eq!(
            movement_target_for(
                &[KeyCode::KeyW, KeyCode::KeyD],
                HexOrientation::Flat,
                EdgeDirection::FLAT_NORTH_WEST,
                EdgeDirection::FLAT_NORTH_WEST,
            ),
            Some(hexx::Hex::ZERO.neighbor(EdgeDirection::FLAT_NORTH_WEST)),
        );
    }

    #[test]
    fn flat_up_right_does_not_take_open_south_west_edge() {
        assert_eq!(
            movement_target_for(
                &[KeyCode::KeyW, KeyCode::KeyD],
                HexOrientation::Flat,
                EdgeDirection::FLAT_SOUTH_WEST,
                EdgeDirection::FLAT_SOUTH_WEST,
            ),
            None,
        );
    }

    #[test]
    fn keys_follow_screen_directions_for_both_layouts() {
        let cases: &[(HexOrientation, &[KeyCode], EdgeDirection)] = &[
            (
                HexOrientation::Flat,
                &[KeyCode::KeyA],
                EdgeDirection::FLAT_SOUTH,
            ),
            (
                HexOrientation::Flat,
                &[KeyCode::KeyD],
                EdgeDirection::FLAT_NORTH,
            ),
            (
                HexOrientation::Flat,
                &[KeyCode::KeyW, KeyCode::KeyA],
                EdgeDirection::FLAT_SOUTH_WEST,
            ),
            (
                HexOrientation::Flat,
                &[KeyCode::KeyW, KeyCode::KeyD],
                EdgeDirection::FLAT_NORTH_WEST,
            ),
            (
                HexOrientation::Flat,
                &[KeyCode::KeyS, KeyCode::KeyA],
                EdgeDirection::FLAT_SOUTH_EAST,
            ),
            (
                HexOrientation::Flat,
                &[KeyCode::KeyS, KeyCode::KeyD],
                EdgeDirection::FLAT_NORTH_EAST,
            ),
            (
                HexOrientation::Pointy,
                &[KeyCode::KeyW],
                EdgeDirection::POINTY_WEST,
            ),
            (
                HexOrientation::Pointy,
                &[KeyCode::KeyS],
                EdgeDirection::POINTY_EAST,
            ),
            (
                HexOrientation::Pointy,
                &[KeyCode::KeyW, KeyCode::KeyA],
                EdgeDirection::POINTY_SOUTH_WEST,
            ),
            (
                HexOrientation::Pointy,
                &[KeyCode::KeyW, KeyCode::KeyD],
                EdgeDirection::POINTY_NORTH_WEST,
            ),
            (
                HexOrientation::Pointy,
                &[KeyCode::KeyS, KeyCode::KeyA],
                EdgeDirection::POINTY_SOUTH_EAST,
            ),
            (
                HexOrientation::Pointy,
                &[KeyCode::KeyS, KeyCode::KeyD],
                EdgeDirection::POINTY_NORTH_EAST,
            ),
        ];

        for &(orientation, keys, direction) in cases {
            assert_eq!(
                movement_target_for(keys, orientation, direction, direction),
                Some(hexx::Hex::ZERO.neighbor(direction)),
                "{orientation:?} {keys:?}",
            );
        }
    }

    #[test]
    fn open_edge_without_a_neighbor_does_not_move_player() {
        assert_eq!(
            movement_target_for(
                &[KeyCode::KeyW, KeyCode::KeyD],
                HexOrientation::Flat,
                EdgeDirection::FLAT_NORTH_WEST,
                EdgeDirection::FLAT_SOUTH,
            ),
            None,
        );
    }

    #[test]
    fn destination_wall_blocks_movement() {
        let mut app = App::new();
        let mut input = ButtonInput::default();
        input.press(KeyCode::KeyW);
        input.press(KeyCode::KeyD);
        app.insert_resource(input);
        app.add_systems(Update, player_input);

        let direction = EdgeDirection::FLAT_NORTH_WEST;
        let mut maze = Maze::new();
        maze.insert(hexx::Hex::ZERO);
        maze.insert(hexx::Hex::ZERO.neighbor(direction));
        assert!(maze.remove_tile_wall(&hexx::Hex::ZERO, direction).is_ok());
        app.world_mut()
            .spawn((CurrentFloor, MazeConfig::default(), maze));
        let player = app
            .world_mut()
            .spawn((Player, CurrentPosition::default()))
            .id();

        app.update();

        assert_eq!(
            app.world()
                .get::<MovementTarget>(player)
                .map(|target| target.0),
            Some(None)
        );
    }

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
