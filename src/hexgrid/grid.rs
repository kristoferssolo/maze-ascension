use bevy::{
    color::palettes::css::{BLACK, GREEN, RED},
    prelude::*,
    utils::hashbrown::HashMap,
};

use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    path::PathBuilder,
    plugin::ShapePlugin,
};
use rand::{prelude::SliceRandom, rngs::ThreadRng, thread_rng};

use super::tile::{AxialCoord, Tile};

const DIRECTIONS: [AxialCoord; 6] = [
    AxialCoord { q: 1, r: 0 },  // Right
    AxialCoord { q: 1, r: -1 }, // Top-right
    AxialCoord { q: 0, r: -1 }, // Top-left
    AxialCoord { q: -1, r: 0 }, // Left
    AxialCoord { q: -1, r: 1 }, // Bottom-left
    AxialCoord { q: 0, r: 1 },  // Bottom-right
];

pub struct HexGrid;

impl Plugin for HexGrid {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin);
        app.add_systems(Startup, setup_system);
    }
}

pub(super) fn setup_system(mut commands: Commands) {
    let radius = 7;
    let mut grid = generate_hex_grix(radius);

    let start_coord = AxialCoord::new(-radius, 0);
    let end_coord = AxialCoord::new(radius, 0);

    let mut rng = thread_rng();
    generate_maze(&mut grid, start_coord, &mut rng);

    render_maze(&mut commands, &mut grid, radius, start_coord, end_coord);
}

fn generate_hex_grix(radius: i32) -> HashMap<AxialCoord, Tile> {
    let mut grid = HashMap::new();

    for q in -radius..=radius {
        let r1 = (-radius).max(-q - radius);
        let r2 = radius.min(-q + radius);

        for r in r1..=r2 {
            let coord = AxialCoord::new(q, r);
            let tile = Tile {
                position: coord,
                walls: [true; 6],
                visited: false,
            };
            grid.insert(coord, tile);
        }
    }
    grid
}

fn add_hex_tile(
    commands: &mut Commands,
    position: Vec2,
    size: f32,
    walls: [bool; 6],
    fill_color: Color,
) {
    let hex_points = (0..6)
        .map(|i| {
            let angle_deg = 60. * i as f32 - 30.;
            let angle_rad = angle_deg.to_radians();
            Vec2::new(size * angle_rad.cos(), size * angle_rad.sin())
        })
        .collect::<Vec<Vec2>>();

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(hex_points[0]);
    for point in &hex_points[1..] {
        path_builder.line_to(*point);
    }
    path_builder.close();
    let hexagon = path_builder.build();

    // Create the hexagon fill
    commands.spawn((
        ShapeBundle {
            path: hexagon,
            spatial: SpatialBundle {
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            },
            ..default()
        },
        Fill::color(fill_color),
    ));

    // Draw walls
    for i in 0..6 {
        if walls[i] {
            let start = hex_points[i];
            let end = hex_points[(i + 1) % 6];
            let mut line_builder = PathBuilder::new();
            line_builder.move_to(start);
            line_builder.line_to(end);
            let line = line_builder.build();

            commands.spawn((
                ShapeBundle {
                    path: line,
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(position.x, position.y, 1.),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(BLACK, 2.),
            ));
        }
    }
}

fn generate_maze(
    grid: &mut HashMap<AxialCoord, Tile>,
    current_coord: AxialCoord,
    rng: &mut ThreadRng,
) {
    {
        let current_tile = grid.get_mut(&current_coord).unwrap();
        current_tile.visit();
    }

    let mut directions = DIRECTIONS.clone();
    directions.shuffle(rng);

    for (i, direction) in directions.iter().enumerate() {
        let neightbor_coord = AxialCoord {
            q: current_coord.q + direction.q,
            r: current_coord.r + direction.r,
        };

        if let Some(neightbor_tile) = grid.get(&neightbor_coord) {
            if !neightbor_tile.visited {
                // Remove the wall between current_tile and neighbor_tile
                {
                    let current_tile = grid.get_mut(&current_coord).unwrap();
                    current_tile.walls[i] = false;
                }
                {
                    let neightbor_tile = grid.get_mut(&neightbor_coord).unwrap();
                    neightbor_tile.walls[opposite_wall(i)] = false;
                }
                // Recurse with the neighbor tile
                generate_maze(grid, neightbor_coord, rng);
            }
        }
    }
}

fn render_maze(
    commands: &mut Commands,
    grid: &mut HashMap<AxialCoord, Tile>,
    radius: i32,
    start_coord: AxialCoord,
    end_coord: AxialCoord,
) {
    let hex_size = 30.;
    let hex_height = hex_size * 2.;
    let hex_width = (3.0_f32).sqrt() * hex_size;

    for tile in grid.values() {
        let (q, r) = (tile.position.q, tile.position.r);
        let x = hex_width * (q as f32 + r as f32 / 2.);
        let y = hex_height * (r as f32 * 3. / 4.);
        let mut fill_color = Color::srgb(0.8, 0.8, 0.8);
        if tile.position == start_coord {
            fill_color = GREEN.into();
        } else if tile.position == end_coord {
            fill_color = RED.into();
        }

        add_hex_tile(commands, Vec2::new(x, y), hex_size, tile.walls, fill_color);
    }
}

fn opposite_wall(index: usize) -> usize {
    (index + 3) % 6
}
