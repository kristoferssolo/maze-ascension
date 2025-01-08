mod assets;
pub mod commands;
pub mod components;
pub mod coordinates;
pub mod errors;
pub mod resources;
mod systems;

use bevy::prelude::*;
use commands::SpawnMaze;
pub use resources::GlobalMazeConfig;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GlobalMazeConfig>()
        .add_plugins(systems::plugin);
}

pub fn spawn_level_command(world: &mut World) {
    SpawnMaze::default().apply(world);
}
