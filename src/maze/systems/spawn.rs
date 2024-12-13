use crate::{
    floor::components::Floor,
    maze::{
        assets::MazeAssets,
        components::{Maze, MazeConfig, Tile, Wall},
        resources::GlobalMazeConfig,
    },
};
use bevy::prelude::*;
use hexlab::prelude::*;
use hexx::HexOrientation;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6};

pub(super) fn spawn_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) {
    let maze = MazeBuilder::new()
        .with_radius(maze_config.radius)
        .with_seed(maze_config.seed)
        .with_generator(GeneratorType::RecursiveBacktracking)
        .build()
        .expect("Something went wrong while creating maze");

    let assets = MazeAssets::new(meshes, materials, global_config);
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
                spawn_single_hex_tile(parent, &assets, tile, maze_config, global_config)
            }
        });
}

pub(super) fn spawn_single_hex_tile(
    parent: &mut ChildBuilder,
    assets: &MazeAssets,
    tile: &HexTile,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) {
    let world_pos = tile.to_vec3(&maze_config.layout);
    let rotation = match maze_config.layout.orientation {
        HexOrientation::Pointy => Quat::from_rotation_y(0.0),
        HexOrientation::Flat => Quat::from_rotation_y(FRAC_PI_6), // 30 degrees rotation
    };

    parent
        .spawn((
            Name::new(format!("Hex {}", tile)),
            Tile,
            Mesh3d(assets.hex_mesh.clone()),
            MeshMaterial3d(assets.hex_material.clone()),
            Transform::from_translation(world_pos).with_rotation(rotation),
        ))
        .with_children(|parent| spawn_walls(parent, assets, tile.walls(), global_config));
}

fn spawn_walls(
    parent: &mut ChildBuilder,
    assets: &MazeAssets,
    walls: &Walls,
    global_config: &GlobalMazeConfig,
) {
    let z_rotation = Quat::from_rotation_z(-FRAC_PI_2);
    let y_offset = global_config.height / 2.;

    for i in 0..6 {
        if !walls.contains(i) {
            continue;
        }

        let wall_angle = -FRAC_PI_3 * i as f32;

        let x_offset = global_config.wall_offset() * f32::cos(wall_angle);
        let z_offset = global_config.wall_offset() * f32::sin(wall_angle);
        let pos = Vec3::new(x_offset, y_offset, z_offset);

        let x_rotation = Quat::from_rotation_x(wall_angle + FRAC_PI_2);
        let final_rotation = z_rotation * x_rotation;

        spawn_single_wall(parent, assets, final_rotation, pos);
    }
}

fn spawn_single_wall(parent: &mut ChildBuilder, assets: &MazeAssets, rotation: Quat, offset: Vec3) {
    parent.spawn((
        Name::new("Wall"),
        Wall,
        Mesh3d(assets.wall_mesh.clone()),
        MeshMaterial3d(assets.wall_material.clone()),
        Transform::from_translation(offset).with_rotation(rotation),
    ));
}
