use super::{
    common::generate_maze,
    spawn::{spawn_maze_tiles, spawn_single_hex_tile},
};
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

pub(super) fn update_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    maze_query: &mut Query<(Entity, &Floor, &mut Maze)>,
    floor: u8,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) -> MazeResult<()> {
    let (entity, _, mut maze) = maze_query
        .iter_mut()
        .find(|(_, f, _)| f.0 == floor)
        .ok_or(MazeError::FloorNotFound(floor))?;

    maze.0 = generate_maze(maze_config)?;

    commands.entity(entity).despawn_descendants();
    let assets = MazeAssets::new(meshes, materials, global_config);
    spawn_maze_tiles(
        commands,
        entity,
        &maze.0,
        &assets,
        maze_config,
        global_config,
    );

    commands.entity(entity).insert(maze_config.clone());

    Ok(())
}
