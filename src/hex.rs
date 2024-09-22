use bevy::{color::palettes::css::BLACK, prelude::*};

use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    path::PathBuilder,
    plugin::ShapePlugin,
};
use rand::{thread_rng, Rng};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ShapePlugin);
    app.add_systems(Startup, setup_system);
}

fn setup_system(mut commands: Commands) {
    let radius = 5;
    let hex_positions = generate_hex_grix(radius);

    let hex_size = 30.;
    let hex_height = hex_size * 2.;
    let hex_width = (3.0_f32).sqrt() * hex_size;

    for (q, r) in hex_positions {
        let x = hex_width * (q as f32 + r as f32 / 2.);
        let y = hex_height * (r as f32 * 3. / 4.);
        let mut rng = thread_rng();
        let walls: [bool; 6] = [
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ];

        add_hex_tile(&mut commands, Vec2::new(x, y), hex_size, walls);
    }
}

fn generate_hex_grix(radius: i32) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();

    for q in -radius..=radius {
        let r1 = (-radius).max(-q - radius);
        let r2 = radius.min(-q + radius);

        for r in r1..=r2 {
            positions.push((q, r));
        }
    }
    positions
}

fn add_hex_tile(commands: &mut Commands, position: Vec2, size: f32, walls: [bool; 6]) {
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

    commands.spawn((
        ShapeBundle {
            path: hexagon,
            spatial: SpatialBundle {
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            },
            ..default()
        },
        Fill::color(Color::srgb(0.8, 0.8, 0.8)),
    ));

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
