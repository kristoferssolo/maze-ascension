mod assets;
pub mod components;
pub mod errors;
pub mod events;
pub mod resources;
mod systems;
mod triggers;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::HexMaze;
use events::{DespawnMaze, RespawnMaze, SpawnMaze};
pub use resources::{GlobalMazeConfig, MazePluginLoaded};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GlobalMazeConfig>()
        .add_event::<SpawnMaze>()
        .add_event::<RespawnMaze>()
        .add_event::<DespawnMaze>()
        .register_type::<HexMaze>()
        .add_plugins((systems::plugin, triggers::plugin));
}

pub fn spawn_level_command(world: &mut World) {
    let _ = world.run_system_once(systems::setup::setup);
    world.insert_resource(MazePluginLoaded);
}
