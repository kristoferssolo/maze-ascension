use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6};

use bevy::prelude::*;
use hexlab::prelude::*;
use hexx::HexOrientation;

use crate::maze::{
    assets::MazeAssets,
    components::{MazeTile, MazeWall},
    MazeConfig,
};

pub(super) fn spawn_single_hex_tile(
    parent: &mut ChildBuilder,
    assets: &MazeAssets,
    tile: &HexTile,
    config: &MazeConfig,
) {
    let world_pos = tile.to_vec3(&config.layout);
    let rotation = match config.layout.orientation {
        HexOrientation::Pointy => Quat::from_rotation_y(0.0),
        HexOrientation::Flat => Quat::from_rotation_y(FRAC_PI_6), // 30 degrees rotation
    };

    parent
        .spawn((
            Name::new(format!("Hex {}", tile.to_string())),
            MazeTile,
            PbrBundle {
                mesh: assets.hex_mesh.clone(),
                material: assets.hex_material.clone(),
                transform: Transform::from_translation(world_pos).with_rotation(rotation),
                ..default()
            },
        ))
        .with_children(|parent| spawn_walls(parent, assets, config, &tile.walls()));
}

fn spawn_walls(parent: &mut ChildBuilder, assets: &MazeAssets, config: &MazeConfig, walls: &Walls) {
    let z_rotation = Quat::from_rotation_z(-FRAC_PI_2);
    let y_offset = config.height / 2.;

    for i in 0..6 {
        if !walls.contains(i) {
            continue;
        }

        let wall_angle = -FRAC_PI_3 * i as f32;

        let x_offset = config.wall_offset() * f32::cos(wall_angle);
        let z_offset = config.wall_offset() * f32::sin(wall_angle);
        let pos = Vec3::new(x_offset, y_offset, z_offset);

        let x_rotation = Quat::from_rotation_x(wall_angle + FRAC_PI_2);
        let final_rotation = z_rotation * x_rotation;

        spawn_single_wall(parent, assets, final_rotation, pos);
    }
}

fn spawn_single_wall(
    parent: &mut ChildBuilder,
    asstets: &MazeAssets,
    rotation: Quat,
    offset: Vec3,
) {
    parent.spawn((
        Name::new("Wall"),
        MazeWall,
        PbrBundle {
            mesh: asstets.wall_mesh.clone(),
            material: asstets.wall_material.clone(),
            transform: Transform::from_translation(offset).with_rotation(rotation),
            ..default()
        },
    ));
}
