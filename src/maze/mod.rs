mod assets;
pub mod components;
pub mod errors;
pub mod events;
pub mod resources;
mod systems;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::Maze;
use events::MazeEvent;
pub use resources::{GlobalMazeConfig, MazePluginLoaded};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GlobalMazeConfig>()
        .add_event::<MazeEvent>()
        .register_type::<Maze>()
        .add_plugins(systems::plugin);
}

pub fn spawn_level_command(world: &mut World) {
    let _ = world.run_system_once(systems::setup::setup);
    world.insert_resource(MazePluginLoaded);
}
