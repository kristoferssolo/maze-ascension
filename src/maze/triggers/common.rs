use crate::maze::{
    components::MazeConfig,
    errors::{MazeError, MazeResult},
};
use hexlab::{GeneratorType, HexMaze, MazeBuilder};

pub fn generate_maze(config: &MazeConfig) -> MazeResult<HexMaze> {
    MazeBuilder::new()
        .with_radius(config.radius)
        .with_seed(config.seed)
        .with_generator(GeneratorType::RecursiveBacktracking)
        .build()
        .map_err(|_| MazeError::generation_failed(config.radius, config.seed))
}
