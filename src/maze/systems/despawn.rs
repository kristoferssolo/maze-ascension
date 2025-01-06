//! Maze despawning functionality.
//!
//! Module handles the cleanup of maze entities when they need to be removed,
//! ensuring proper cleanup of both the maze and all its child entities.
use crate::{floor::components::Floor, maze::commands::DespawnMaze};

use bevy::prelude::*;

/// Despawns a maze and all its associated entities for a given floor.
pub fn despawn_maze(
    In(DespawnMaze { floor }): In<DespawnMaze>,
    mut commands: Commands,
    query: Query<(Entity, &Floor)>,
) {
    match query.iter().find(|(_, f)| f.0 == floor) {
        Some((entity, _)) => commands.entity(entity).despawn_recursive(),
        _ => warn!("Floor {} not found for removal", floor),
    }
}
