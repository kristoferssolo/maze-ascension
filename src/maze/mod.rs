use bevy::{ecs::system::RunSystemOnce, prelude::*};
use events::RecreateMazeEvent;
mod assets;
pub mod components;
pub mod errors;
pub mod events;
pub mod resources;
mod systems;

pub use resources::{GlobalMazeConfig, MazePluginLoaded};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GlobalMazeConfig>()
        .add_event::<RecreateMazeEvent>()
        .add_plugins(systems::plugin);
}

pub fn spawn_level_command(world: &mut World) {
    world.insert_resource(MazePluginLoaded);
    let _ = world.run_system_once(systems::setup::setup);
}
