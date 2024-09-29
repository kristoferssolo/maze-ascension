use std::usize;

use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, SILVER},
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
    utils::hashbrown::HashMap,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    path::PathBuilder,
    plugin::ShapePlugin,
};
use hexx::{EdgeDirection, Hex};
use rand::{prelude::SliceRandom, rngs::ThreadRng, thread_rng};

use super::{
    resource::{Layout, MazeConfig},
    tile::{Tile, TileBundle, Walls},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ShapePlugin, WireframePlugin));
    app.init_resource::<MazeConfig>();
    app.init_resource::<Layout>();
    app.insert_resource(WireframeConfig {
        global: false,
        ..default()
    });
}

pub(super) fn spawn_hex_grid(mut commands: Commands, config: Res<MazeConfig>) {
    let radius = config.radius as i32;

    for q in -radius..=radius {
        let r1 = (-radius).max(-q - radius);
        let r2 = radius.min(-q + radius);
        for r in r1..=r2 {
            let tile = Tile::new(q, r);
            commands.spawn((
                Name::new(format!("Tile {}", &tile.to_string())),
                TileBundle {
                    hex: tile,
                    ..default()
                },
            ));
        }
    }
}

pub(super) fn generate_maze(
    mut commands: Commands,
    query: Query<(Entity, &Tile, &Walls)>,
    config: Res<MazeConfig>,
) {
    let mut tiles = query
        .into_iter()
        .map(|(entity, tile, walls)| (tile.hex, (entity, tile.clone(), walls.clone())))
        .collect();

    let mut rng = thread_rng();
    recursive_maze(&mut tiles, config.start_pos, &mut rng);

    for (entity, tile, walls) in tiles.values() {
        commands
            .entity(*entity)
            .insert(tile.clone())
            .insert(walls.clone());
    }
}

fn recursive_maze(
    tiles: &mut HashMap<Hex, (Entity, Tile, Walls)>,
    current_hex: Hex,
    rng: &mut ThreadRng,
) {
    {
        let (_, tile, _) = tiles.get_mut(&current_hex).unwrap();
        tile.visit();
    }

    let mut directions = EdgeDirection::ALL_DIRECTIONS;
    directions.shuffle(rng);

    for direction in directions.into_iter() {
        let neighbor_hex = current_hex + direction;
        if let Some((_, neighbor_tile, _)) = tiles.get(&neighbor_hex) {
            if !neighbor_tile.visited {
                remove_wall_between(tiles, current_hex, neighbor_hex, direction);
                recursive_maze(tiles, neighbor_hex, rng);
            }
        }
    }
}

fn remove_wall_between(
    tiles: &mut HashMap<Hex, (Entity, Tile, Walls)>,
    current_hex: Hex,
    neighbor_hex: Hex,
    direction: EdgeDirection,
) {
    {
        let (_, _, walls) = tiles.get_mut(&current_hex).unwrap();
        walls.0[direction.index() as usize] = false;
    }
    {
        let (_, _, walls) = tiles.get_mut(&neighbor_hex).unwrap();
        walls.0[direction.const_neg().index() as usize] = false;
    }
}

fn add_hex_tile(
    commands: &mut Commands,
    position: Vec3,
    size: f32,
    tile: &Tile,
    walls: &Walls,
    fill_color: Color,
    layout: &Layout,
) {
    let hex_points = tile
        .hex
        .all_vertices()
        .into_iter()
        .map(|v| {
            let mut layout = layout.clone();
            layout.origin = position.xy();
            layout.hex_size = Vec2::splat(size);
            layout.hex_to_world_pos(v.origin + v.direction)
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
    commands
        .spawn((
            ShapeBundle {
                path: hexagon,
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(position.x, position.y, 0.),
                    ..default()
                },
                ..default()
            },
            Fill::color(fill_color),
        ))
        .with_children(|p| {
            p.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: tile.to_string(),
                        style: TextStyle {
                            font_size: 16.,
                            color: Color::BLACK,
                            ..default()
                        },
                    }],
                    ..default()
                },
                transform: Transform::from_xyz(position.x * 2., position.y * 2., 1.),
                ..default()
            });
        });

    // Draw walls
    for direction in EdgeDirection::iter() {
        let idx = direction.index() as usize;
        if walls[idx] {
            let start = hex_points[idx];
            let end = hex_points[(idx + 1) % 6];
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

pub(super) fn render_maze(
    mut commands: Commands,
    query: Query<(&Tile, &mut Walls)>,
    layout: Res<Layout>,
    config: Res<MazeConfig>,
) {
    for (tile, walls) in query.iter() {
        let world_pos = layout.hex_to_world_pos(tile.hex).extend(0.);
        let fill_color = match tile.hex {
            pos if pos == config.start_pos => GREEN.into(),
            pos if pos == config.end_pos => RED.into(),
            _ => Color::srgb(0.8, 0.8, 0.8),
        };
        add_hex_tile(
            &mut commands,
            world_pos,
            config.size,
            tile,
            walls,
            fill_color,
            &layout,
        );
    }
}
