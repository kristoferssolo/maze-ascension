use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::{
    resource::{Layout, MazeConfig},
    tile::Tile,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_light);
}

pub(super) fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Name::new("Light Source"),
        PointLightBundle {
            transform: Transform::from_xyz(0., 50., 0.),
            ..default()
        },
    ));
}

pub(super) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
    layout: Res<Layout>,
) {
    let radius = config.radius as i32;

    let rotation = Quat::from_rotation_x(-FRAC_PI_2);
    let material = materials.add(Color::WHITE);
    let prism_mesh = generate_hex_prism_mesh(layout.hex_size.x, config.height);
    let mesh = meshes.add(prism_mesh);

    commands
        .spawn((
            Name::new("Floor"),
            SpatialBundle {
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
        ))
        .with_children(|p| {
            for q in -radius..=radius {
                let r1 = (-radius).max(-q - radius);
                let r2 = radius.min(-q + radius);
                for r in r1..=r2 {
                    let tile = Tile::new(q, r);
                    let pos = tile.to_vec3(&layout);
                    p.spawn((
                        Name::new(format!("Hex {}", &tile.to_string())),
                        PbrBundle {
                            mesh: mesh.clone(),
                            material: material.clone(),
                            transform: Transform::from_translation(pos).with_rotation(rotation),
                            ..default()
                        },
                    ));
                }
            }
        });
}

fn generate_hex_prism_mesh(radius: f32, depth: f32) -> Mesh {
    let hexagon = RegularPolygon {
        sides: 6,
        circumcircle: Circle::new(radius),
    };
    let prism_shape = Extrusion::new(hexagon, depth);

    Mesh::from(prism_shape)
}
