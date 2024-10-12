use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};

use super::{grid, prism};

#[derive(Default)]
pub(crate) struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(prism::plugin);
        app.add_plugins(grid::plugin);
    }
}

impl Command for MazePlugin {
    fn apply(self, world: &mut World) {
        // world.run_system_once(spawn_hex_grid);
        // world.run_system_once(generate_maze);
        // world.run_system_once(render_maze);
        world.run_system_once(prism::setup);
    }
}
