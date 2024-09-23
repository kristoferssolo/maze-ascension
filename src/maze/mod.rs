use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use grid::{generate_maze, plugin, render_maze, spawn_hex_grid};
pub mod grid;
pub mod resource;
pub mod tile;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(plugin);
    }
}

impl Command for MazePlugin {
    fn apply(self, world: &mut World) {
        world.run_system_once(spawn_hex_grid);
        world.run_system_once(generate_maze);
        world.run_system_once(render_maze);
    }
}

pub fn spawn_grid(world: &mut World) {
    MazePlugin.apply(world);
}
