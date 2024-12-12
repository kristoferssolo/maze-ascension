use bevy::prelude::*;
use hexlab::{GeneratorType, MazeBuilder};

use crate::maze::{
    assets::MazeAssets,
    components::{Floor, Maze},
    MazeConfig,
};

use super::spawn::spawn_single_hex_tile;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
) {
    setup_maze(&mut commands, &mut meshes, &mut materials, &config);
}

pub(super) fn setup_maze(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &MazeConfig,
) {
    let maze = MazeBuilder::new()
        .with_radius(config.radius)
        .with_seed(config.seed)
        .with_generator(GeneratorType::RecursiveBacktracking)
        .build()
        .expect("Something went wrong while creating maze");

    let assets = MazeAssets::new(meshes, materials, config);
    commands
        .spawn((
            Name::new("Floor"),
            Maze(maze.clone()),
            Floor(1),
            Transform::from_translation(Vec3::ZERO),
            Visibility::Visible,
        ))
        .with_children(|parent| {
            for tile in maze.values() {
                spawn_single_hex_tile(parent, &assets, tile, config)
            }
        });
}
