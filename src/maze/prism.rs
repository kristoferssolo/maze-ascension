use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_light);
}

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
struct Prism {
    radius: f32,
    height: f32,
}

struct PrismParams {
    position: Vec3,
    radius: f32,
    height: f32,
}

impl From<Vec3> for PrismParams {
    fn from(value: Vec3) -> Self {
        Self {
            position: value,
            ..default()
        }
    }
}

impl Default for PrismParams {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            radius: 2.,
            height: 4.,
        }
    }
}

pub(super) fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Name::new("Light Source"),
        PointLightBundle {
            point_light: PointLight { ..default() },
            transform: Transform::from_xyz(5., 10., 5.),
            ..default()
        },
    ));
}

pub(super) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let prism_material = materials.add(Color::WHITE);

    let prisms: Vec<PrismParams> = vec![Vec3::ZERO.into()];

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
                transform: Transform::from_translation(params.position)
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
