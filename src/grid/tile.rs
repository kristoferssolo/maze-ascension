use super::{
    direction::HexDirection,
    grid::{Grid, GridSettings},
};
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    utils::hashbrown::HashMap,
};
use hexx::{Hex, HexLayout, PlaneMeshBuilder};

pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tile {
    pub position: Hex,
    pub walls: HashMap<HexDirection, bool>,
}

pub fn spawn_tiles(
    config: In<GridSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>,
) {
    let default_material = materials.add(Color::WHITE);

    let mesh = hexagonal_plane(&grid.layout);
    let mesh_handle = meshes.add(mesh);

    for hex_pos in Hex::ZERO.range(config.radius) {
        let world_pos = grid.layout.hex_to_world_pos(hex_pos);
        commands.spawn((
            Name::new(format!("Tile: ({}, {})", world_pos.x, world_pos.y)),
            ColorMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                transform: Transform::from_xyz(world_pos.x, world_pos.y, 0.),
                material: default_material.clone(),
                ..default()
            },
            Tile {
                position: hex_pos,
                ..default()
            },
        ));
    }
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.9)) // border
        .center_aligned()
        .build();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
