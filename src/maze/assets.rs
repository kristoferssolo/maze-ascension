use super::resources::GlobalMazeConfig;
use crate::theme::{palette::rose_pine::RosePine, prelude::ColorScheme};
use bevy::{prelude::*, utils::HashMap};
use std::f32::consts::FRAC_PI_2;
use strum::IntoEnumIterator;

const WALL_OVERLAP_MODIFIER: f32 = 1.25;
const HEX_SIDES: u32 = 6;
const WHITE_EMISSION_INTENSITY: f32 = 10.;

pub struct MazeAssets {
    pub hex_mesh: Handle<Mesh>,
    pub wall_mesh: Handle<Mesh>,
    pub hex_material: Handle<StandardMaterial>,
    pub wall_material: Handle<StandardMaterial>,
    pub custom_materials: HashMap<RosePine, Handle<StandardMaterial>>,
}

impl MazeAssets {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        global_config: &GlobalMazeConfig,
    ) -> Self {
        let custom_materials = RosePine::iter()
            .map(|color| (color, materials.add(color.to_standart_material())))
            .collect();
        Self {
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

pub fn white_material() -> StandardMaterial {
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
