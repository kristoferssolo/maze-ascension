use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::{
    resources::{HEX_SIZE, WALL_SIZE},
    MazeConfig,
};

pub(crate) struct MazeAssets {
    pub(crate) hex_mesh: Handle<Mesh>,
    pub(crate) wall_mesh: Handle<Mesh>,
    pub(crate) hex_material: Handle<StandardMaterial>,
    pub(crate) wall_material: Handle<StandardMaterial>,
}

pub(crate) fn create_base_assets(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &MazeConfig,
) -> MazeAssets {
    MazeAssets {
        hex_mesh: meshes.add(generate_hex_mesh(HEX_SIZE, config.height)),
        wall_mesh: meshes.add(generate_square_mesh(HEX_SIZE)),
        hex_material: materials.add(white_material()),
        wall_material: materials.add(Color::BLACK),
    }
}

fn generate_hex_mesh(radius: f32, depth: f32) -> Mesh {
    let hexagon = RegularPolygon {
        sides: 6,
        circumcircle: Circle::new(radius),
    };
    let prism_shape = Extrusion::new(hexagon, depth);
    let rotation = Quat::from_rotation_x(FRAC_PI_2);

    Mesh::from(prism_shape).rotated_by(rotation)
}

fn generate_square_mesh(depth: f32) -> Mesh {
    let square = Rectangle::new(WALL_SIZE, WALL_SIZE);
    let rectangular_prism = Extrusion::new(square, depth);
    let rotation = Quat::from_rotation_x(FRAC_PI_2);

    Mesh::from(rectangular_prism).rotated_by(rotation)
}

fn white_material() -> StandardMaterial {
    let val = 10.;
    StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::new(val, val, val, val),
        ..default()
    }
}
