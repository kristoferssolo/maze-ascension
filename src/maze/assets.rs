use crate::theme::palette::rose_pine::{LOVE, PINE};

use super::resources::GlobalMazeConfig;
use bevy::{prelude::*, utils::HashMap};
use std::f32::consts::FRAC_PI_2;

const WALL_OVERLAP_MODIFIER: f32 = 1.25;
const HEX_SIDES: u32 = 6;
const WHITE_EMISSION_INTENSITY: f32 = 10.;

pub(crate) struct MazeAssets {
    pub(crate) hex_mesh: Handle<Mesh>,
    pub(crate) wall_mesh: Handle<Mesh>,
    pub(crate) hex_material: Handle<StandardMaterial>,
    pub(crate) wall_material: Handle<StandardMaterial>,
    pub(crate) custom_materials: HashMap<String, Handle<StandardMaterial>>,
}

impl MazeAssets {
    pub(crate) fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        global_config: &GlobalMazeConfig,
    ) -> MazeAssets {
        let mut custom_materials = HashMap::new();
        custom_materials.extend(vec![
            ("LOVE".to_string(), materials.add(red_material())),
            ("PINE".to_string(), materials.add(blue_material())),
        ]);
        MazeAssets {
            hex_mesh: meshes.add(generate_hex_mesh(
                global_config.hex_size,
                global_config.height,
            )),
            wall_mesh: meshes.add(generate_square_mesh(
                global_config.hex_size + global_config.wall_size() / WALL_OVERLAP_MODIFIER,
                global_config.wall_size(),
            )),
            hex_material: materials.add(white_material()),
            wall_material: materials.add(Color::BLACK),
            custom_materials,
        }
    }
}

fn generate_hex_mesh(radius: f32, depth: f32) -> Mesh {
    let hexagon = RegularPolygon {
        sides: HEX_SIDES,
        circumcircle: Circle::new(radius),
    };
    let prism_shape = Extrusion::new(hexagon, depth);
    let rotation = Quat::from_rotation_x(FRAC_PI_2);

    Mesh::from(prism_shape).rotated_by(rotation)
}

fn generate_square_mesh(depth: f32, wall_size: f32) -> Mesh {
    let square = Rectangle::new(wall_size, wall_size);
    let rectangular_prism = Extrusion::new(square, depth);
    let rotation = Quat::from_rotation_x(FRAC_PI_2);

    Mesh::from(rectangular_prism).rotated_by(rotation)
}

pub(crate) fn white_material() -> StandardMaterial {
    StandardMaterial {
        emissive: LinearRgba::new(
            WHITE_EMISSION_INTENSITY,
            WHITE_EMISSION_INTENSITY,
            WHITE_EMISSION_INTENSITY,
            WHITE_EMISSION_INTENSITY,
        ),
        ..default()
    }
}

pub(crate) fn red_material() -> StandardMaterial {
    StandardMaterial {
        emissive: LOVE.to_linear(),
        ..default()
    }
}

pub(crate) fn blue_material() -> StandardMaterial {
    StandardMaterial {
        emissive: PINE.to_linear(),
        ..default()
    }
}
