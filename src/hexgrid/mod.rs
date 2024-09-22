use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use bevy_prototype_lyon::plugin::ShapePlugin;
use grid::setup_system;
pub mod direction;
pub mod grid;
pub mod tile;

pub struct HexGrid;

impl Plugin for HexGrid {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin);
    }
}

impl Command for HexGrid {
    fn apply(self, world: &mut World) {
        world.run_system_once(setup_system);
    }
}

pub fn spawn_grid(world: &mut World) {
    HexGrid.apply(world);
}
