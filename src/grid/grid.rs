use super::tile::{draw_walls, spawn_tiles};
use bevy::{
    color::palettes::css::{BLACK, DARK_CYAN},
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    geometry::GeometryBuilder,
    plugin::ShapePlugin,
    prelude::RegularPolygon,
    shapes::RegularPolygonFeature,
};
use hexx::{HexLayout, HexOrientation};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Msaa::Sample4);
    app.add_plugins(ShapePlugin);
}

pub fn spawn_grid(world: &mut World) {
    world.init_resource::<Grid>();
    world.init_resource::<RotationTimer>();
    GridSettings::default().apply(world);
}

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
#[reflect(Resource)]
struct RotationTimer(Timer);

impl Default for RotationTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once))
    }
}

#[derive(Debug, Reflect)]
pub struct GridSettings {
    pub radius: u32,
}

impl Default for GridSettings {
    fn default() -> Self {
        Self { radius: 10 }
    }
}

impl Command for GridSettings {
    fn apply(self, world: &mut World) {
        // world.run_system_once_with(self, spawn_tiles);
        // world.run_system_once(draw_walls);
        world.run_system_once(draw);
    }
}

fn draw(mut commands: Commands) {
    let shape = RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(200.),
        ..default()
    };

    commands.spawn((
        Name::new("Hexagon"),
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(DARK_CYAN),
        Stroke::new(BLACK, 10.),
    ));
}

#[derive(Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct Grid {
    pub layout: HexLayout,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(20., HexOrientation::Flat)
    }
}

impl Grid {
    pub fn new(hex_size: f32, orientation: HexOrientation) -> Self {
        Self {
            layout: HexLayout {
                hex_size: Vec2::splat(hex_size),
                orientation,
                ..default()
            },
        }
    }
}
