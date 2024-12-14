use crate::{
    floor::components::Floor,
    maze::{
        assets::MazeAssets,
        components::{Maze, MazeConfig},
        errors::{MazeError, MazeResult},
        GlobalMazeConfig,
    },
};
use bevy::prelude::*;
use hexlab::{GeneratorType, MazeBuilder};

use super::spawn::spawn_single_hex_tile;

pub(super) fn update_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    maze_query: &mut Query<(Entity, &Floor, &Children, &mut Maze)>,
    floor: u8,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) -> MazeResult<()> {
    let (entity, _, children, mut maze) = maze_query
        .iter_mut()
        .find(|(_, f, _, _)| f.0 == floor)
        .ok_or(MazeError::FloorNotFound(floor))?;

    let new_maze = MazeBuilder::new()
        .with_radius(maze_config.radius)
        .with_seed(maze_config.seed)
        .with_generator(GeneratorType::RecursiveBacktracking)
        .build()
        .map_err(|_| MazeError::generation_failed(maze_config.radius, maze_config.seed))?;

    let new_maze = Maze(new_maze);

    commands.entity(entity).clear_children();
    for &child in children.iter() {
        commands.entity(child).despawn_recursive();
    }
    let assets = MazeAssets::new(meshes, materials, global_config);
    commands.entity(entity).with_children(|parent| {
        for tile in new_maze.0.values() {
            spawn_single_hex_tile(parent, &assets, tile, maze_config, global_config);
        }
    });

    *maze = new_maze;
    commands.entity(entity).insert(maze_config.clone());
    Ok(())
}
