use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
struct Prism {
    radius: f32,
    height: f32,
}

struct PrismParams {
    positions: Vec3,
    radius: f32,
    height: f32,
}

pub(super) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let prism_material = materials.add(Color::WHITE);

    let prisms = vec![
        PrismParams {
            positions: Vec3::new(-3., 0., 0.),
            radius: 1.,
            height: 2.,
        },
        PrismParams {
            positions: Vec3::new(0., 0., 0.),
            radius: 1.5,
            height: 2.5,
        },
        PrismParams {
            positions: Vec3::new(3., 0., 0.),
            radius: 0.8,
            height: 1.5,
        },
    ];

    for params in prisms {
        let hexagon = RegularPolygon {
            sides: 6,
            circumcircle: Circle::new(params.radius),
        };
        let prism_shape = Extrusion::new(hexagon, params.height);
        let prism_mesh = meshes.add(Mesh::from(prism_shape));

        commands.spawn((
            PbrBundle {
                mesh: prism_mesh,
                material: prism_material.clone(),
                transform: Transform::from_translation(params.positions)
                    .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
                ..default()
            },
            Prism {
                radius: params.radius,
                height: params.height,
            },
        ));
    }
}
