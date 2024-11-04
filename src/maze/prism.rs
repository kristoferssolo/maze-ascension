use bevy::{pbr::UvChannel, prelude::*};
use core::f32;
use hexx::{EdgeDirection, GridEdge, Hex};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6};

use super::{
    resource::{Layout, MazeConfig, HEX_SIZE},
    tile::Tile,
};

pub(super) fn plugin(_app: &mut App) {}
const WALL_SIZE: f32 = 1.0;

pub(super) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
    layout: Res<Layout>,
) {
    let radius = config.radius as i32;

    let assets = create_base_assets(&mut meshes, &mut materials, &config);
    // spawn_single_hex_tile(&mut commands, &assets, &config);
    commands
        .spawn((
            Name::new("Floor"),
            SpatialBundle {
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
        ))
        .with_children(|mut parent| {
            for q in -radius..=radius {
                let r1 = (-radius).max(-q - radius);
                let r2 = radius.min(-q + radius);
                for r in r1..=r2 {
                    let tile = Tile::new(q, r);
                    spawn_single_hex_tile(&mut parent, &tile, &layout, &assets, &config);
                }
            }
        });
}

fn spawn_single_hex_tile(
    parent: &mut ChildBuilder,
    tile: &Tile,
    layout: &Res<Layout>,
    assets: &MazeAssets,
    config: &Res<MazeConfig>,
) {
    let pos = tile.to_vec3(&layout);
    parent
        .spawn((
            Name::new(format!("Hex {}", &tile.to_string())),
            PbrBundle {
                mesh: assets.hex_mesh.clone(),
                material: assets.hex_material.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
        ))
        .with_children(|parent| spawn_walls(parent, assets, config));
}

fn spawn_walls(parent: &mut ChildBuilder, asstets: &MazeAssets, config: &Res<MazeConfig>) {
    let y_offset = config.height / 2.;
    let z_rotation = Quat::from_rotation_z(-FRAC_PI_2);

    for i in 0..6 {
        let wall_angle = FRAC_PI_3 * i as f32;

        let x_offset = (HEX_SIZE - WALL_SIZE) * f32::cos(wall_angle);
        let z_offset = (HEX_SIZE - WALL_SIZE) * f32::sin(wall_angle);
        let pos = Vec3::new(x_offset, y_offset, z_offset);

        let x_rotation = Quat::from_rotation_x(wall_angle + FRAC_PI_2);
        let final_rotation = z_rotation * x_rotation;

        spawn_single_wall(parent, asstets, final_rotation, pos);
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
        PbrBundle {
            mesh: asstets.wall_mesh.clone(),
            material: asstets.wall_material.clone(),
            transform: Transform::from_translation(offset).with_rotation(rotation),
            ..default()
        },
    ));
}

fn create_base_assets(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &Res<MazeConfig>,
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

struct MazeAssets {
    hex_mesh: Handle<Mesh>,
    wall_mesh: Handle<Mesh>,
    hex_material: Handle<StandardMaterial>,
    wall_material: Handle<StandardMaterial>,
}
