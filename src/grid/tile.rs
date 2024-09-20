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
use bevy_prototype_lyon::{entity::ShapeBundle, geometry::GeometryBuilder, shapes::Line};
use hexx::{Hex, HexLayout, PlaneMeshBuilder};
use rand::{thread_rng, Rng};

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

    let mut rng = thread_rng();

    for hex_pos in Hex::ZERO.range(config.radius) {
        let world_pos = grid.layout.hex_to_world_pos(hex_pos);
        let mut walls = HashMap::new();

        for dir in HexDirection::ALL {
            walls.insert(dir, rng.gen_bool(0.5));
        }

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
                walls,
            },
        ));
    }
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        // .with_scale(Vec3::splat(0.9)) // border
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

fn hex_corner_positions(layout: &HexLayout, hex: Hex) -> [Vec2; 6] {
    let center = layout.hex_to_world_pos(hex);
    let mut corners = [Vec2::ZERO; 6];
    for (idx, corner) in corners.iter_mut().enumerate() {
        let andgle_deg = 60. * idx as f32; // FIX:
        let andgle_rad = andgle_deg.to_radians();
        let size = layout.hex_size;
        *corner = center + Vec2::new(size.x * andgle_rad.cos(), size.y * andgle_rad.sin());
    }
    corners
}

pub fn draw_walls(
    mut commands: Commands,
    tile_query: Query<(Entity, &Tile)>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>,
) {
    let default_material = materials.add(Color::BLACK);

    for (entity, tile) in tile_query.iter() {
        let corners = hex_corner_positions(&grid.layout, tile.position);

        for (dir, has_wall) in &tile.walls {
            if *has_wall {
                let direction_idx = *dir as usize;
                let cornder1 = direction_idx;
                let cornder2 = (direction_idx + 1) % 6;

                let start_pos = corners[cornder1];
                let end_pos = corners[cornder2];

                let line = Line(start_pos, end_pos);
                let wall_entity = commands
                    .spawn((
                        Name::new("Wall"),
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&line),
                            material: default_material.clone(),
                            ..default()
                        },
                    ))
                    .id();
                commands.entity(entity).add_child(wall_entity);
            }
        }
    }
}
