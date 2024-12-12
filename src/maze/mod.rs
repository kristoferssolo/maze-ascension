use bevy::{ecs::system::RunSystemOnce, prelude::*};
use events::RecreateMazeEvent;
mod assets;
pub mod components;
pub mod events;
mod resources;
mod systems;

pub use resources::{MazeConfig, MazePluginLoaded};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MazeConfig>()
        .add_event::<RecreateMazeEvent>()
        .add_plugins(systems::plugin);
}

pub fn spawn_level_command(world: &mut World) {
    world.insert_resource(MazePluginLoaded);
    let _ = world.run_system_once(systems::setup::setup);
}
