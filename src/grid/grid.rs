use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    utils::hashbrown::HashMap,
};
use hexx::{Hex, HexLayout, HexOrientation, InsetOptions, MeshInfo, PlaneMeshBuilder};

use super::direction::HexDirection;

#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tile {
    pub position: Hex,
    pub walls: HashMap<HexDirection, bool>,
}

const HEX_SIZE: Vec2 = Vec2::splat(13.0);

#[derive(Debug)]
pub struct SpawnGrid;

impl Command for SpawnGrid {
    fn apply(self, world: &mut World) {
        world.run_system_once(spawn_grid);
    }
}

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        orientation: HexOrientation::Pointy,
        ..default()
    };
    let default_material = materials.add(Color::WHITE);

    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    for hex in Hex::ZERO.range(15) {
        let pos = layout.hex_to_world_pos(hex);
        commands.spawn((
            Name::new("Tile"),
            ColorMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                material: default_material.clone(),
                ..default()
            },
            Tile::default(),
        ));
    }
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.98))
        .center_aligned()
        .build();
    construct_mesh(mesh_info)
}

fn border_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_inset_options(InsetOptions {
            keep_inner_face: false,
            scale: 0.2,
            ..default()
        })
        .center_aligned()
        .build();

    construct_mesh(mesh_info)
}

fn construct_mesh(mesh_info: MeshInfo) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
