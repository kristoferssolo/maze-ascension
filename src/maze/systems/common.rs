//! Common maze generation utilities.
use crate::maze::{components::MazeConfig, errors::MazeError};
use hexlab::prelude::*;

/// Generates a new maze based on the provided configuration.
///
/// This function uses a recursive backtracking algorithm to generate
/// a hexagonal maze with the specified parameters.
///
/// # Arguments
/// - `config` - Configuration parameters for maze generation including radius and seed.
///
/// # Returns
/// `Result<Maze, MazeError>` - The generated maze or an error if generation fails.
///
/// # Errors
/// Returns `MazeError::GenerationFailed` if:
/// - The maze builder fails to create a valid maze
/// - The provided radius or seed results in an invalid configuration
pub fn generate_maze(config: &MazeConfig) -> Result<Maze, MazeError> {
    MazeBuilder::new()
        .with_radius(config.radius)
        .with_seed(config.seed)
        .with_generator(GeneratorType::RecursiveBacktracking)
        .build()
        .map_err(|_| MazeError::generation_failed(config.radius, config.seed))
}
