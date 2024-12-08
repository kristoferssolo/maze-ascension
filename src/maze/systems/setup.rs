use bevy::prelude::*;
use hexlab::{GeneratorType, MazeBuilder};

use crate::maze::{
    assets::create_base_assets, components::MazeFloor, resources::Layout, MazeConfig,
};

use super::spawn::spawn_single_hex_tile;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
    layout: Res<Layout>,
) {
    setup_maze(&mut commands, &mut meshes, &mut materials, &config, &layout);
}

pub(super) fn setup_maze(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &MazeConfig,
    layout: &Layout,
) {
    let maze = MazeBuilder::new()
        .with_radius(config.radius)
        // .with_seed(0)
        .with_generator(GeneratorType::RecursiveBacktracking)
        .build()
        .expect("Something went wrong while creating maze");

    let assets = create_base_assets(meshes, materials, config);
    commands
        .spawn((
            Name::new("Floor"),
            MazeFloor(1),
            SpatialBundle {
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
        ))
        .with_children(|parent| {
            for tile in maze.values() {
                spawn_single_hex_tile(parent, &assets, tile, &layout.0, config.height)
            }
        });
}
