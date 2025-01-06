//! Maze respawning functionality.
//!
//! Module provides the ability to regenerate mazes for existing floors,
//! maintaining the same floor entity but replacing its internal maze structure.

use crate::{
    floor::components::Floor,
    maze::{assets::MazeAssets, commands::RespawnMaze, errors::MazeError, GlobalMazeConfig},
};
use bevy::prelude::*;
use hexlab::Maze;

use super::{common::generate_maze, spawn::spawn_maze_tiles};

/// Respawns a maze for an existing floor with a new configuration.
///
/// # Behavior:
/// - Finds the target floor
/// - Generates a new maze configuration
/// - Cleans up existing maze tiles
/// - Spawns new maze tiles
/// - Updates the floor's configuration
pub fn respawn_maze(
    In(RespawnMaze { floor, config }): In<RespawnMaze>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut maze_query: Query<(Entity, &Floor, &mut Maze)>,
    global_config: Res<GlobalMazeConfig>,
) {
    let (entity, _, mut maze) = match maze_query
        .iter_mut()
        .find(|(_, f, _)| f.0 == floor)
        .ok_or(MazeError::FloorNotFound(floor))
    {
        Ok((entity, floor, maze)) => (entity, floor, maze),
        Err(e) => {
            warn!("Failed to update floor ({floor}). {e}");
            return;
        }
    };

    *maze = match generate_maze(&config) {
        Ok(generated_maze) => generated_maze,
        Err(e) => {
            warn!("Failed to update floor ({floor}). {e}");
            return;
        }
    };

    commands.entity(entity).despawn_descendants();
    let assets = MazeAssets::new(&mut meshes, &mut materials, &global_config);
    spawn_maze_tiles(
        &mut commands,
        entity,
        &maze,
        &assets,
        &config,
        &global_config,
    );

    commands.entity(entity).insert(config.clone());
}
