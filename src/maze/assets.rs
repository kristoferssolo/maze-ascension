//! Maze asset management and generation.
//!
//! Module handles the creation and management of meshes and materials
//! used in the maze visualization, including hexagonal tiles and walls.

use super::resources::GlobalMazeConfig;
use crate::{
    constants::WALL_OVERLAP_MODIFIER,
    theme::{palette::rose_pine::RosePineDawn, prelude::ColorScheme},
};

use bevy::{prelude::*, utils::HashMap};
use std::f32::consts::FRAC_PI_2;
use strum::IntoEnumIterator;

const HEX_SIDES: u32 = 6;
const WHITE_EMISSION_INTENSITY: f32 = 10.;

/// Collection of mesh and material assets used in maze rendering.
///
/// This struct contains all the necessary assets for rendering maze components,
/// including hexagonal tiles, walls, and custom colored materials.
#[derive(Debug)]
pub struct MazeAssets {
    /// Mesh for hexagonal floor tiles
    pub hex_mesh: Handle<Mesh>,
    /// Mesh for wall segments
    pub wall_mesh: Handle<Mesh>,
    /// Default material for hexagonal tiles
    pub hex_material: Handle<StandardMaterial>,
    /// Default material for walls
    pub wall_material: Handle<StandardMaterial>,
    /// Custom materials mapped to specific colors from the RosePineDawn palette
    pub custom_materials: HashMap<RosePineDawn, Handle<StandardMaterial>>,
}

impl MazeAssets {
    /// Creates a new instance of MazeAssets with all necessary meshes and materials.
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        global_config: &GlobalMazeConfig,
    ) -> Self {
        let custom_materials = RosePineDawn::iter()
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

/// Generates a hexagonal mesh for floor tiles.
fn generate_hex_mesh(radius: f32, depth: f32) -> Mesh {
    let hexagon = RegularPolygon {
        sides: HEX_SIDES,
        circumcircle: Circle::new(radius),
    };
    let prism_shape = Extrusion::new(hexagon, depth);
    let rotation = Quat::from_rotation_x(FRAC_PI_2);

    Mesh::from(prism_shape).rotated_by(rotation)
}

/// Generates a square mesh for wall segments.
fn generate_square_mesh(depth: f32, wall_size: f32) -> Mesh {
    let square = Rectangle::new(wall_size, wall_size);
    let rectangular_prism = Extrusion::new(square, depth);
    let rotation = Quat::from_rotation_x(FRAC_PI_2);

    Mesh::from(rectangular_prism).rotated_by(rotation)
}

/// Creates a glowing white material for default tile appearance.
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
