use bevy::{ecs::system::RunSystemOnce, prelude::*};
use events::RecreateMazeEvent;
mod assets;
mod components;
pub mod events;
mod resources;
mod systems;

pub use resources::{MazeConfig, MazePluginLoaded};
use systems::recreation::handle_maze_recreation_event;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MazeConfig>()
        .add_event::<RecreateMazeEvent>()
        .add_systems(Update, handle_maze_recreation_event);
}

pub fn spawn_level_command(world: &mut World) {
    world.insert_resource(MazePluginLoaded);
    world.run_system_once(systems::setup::setup);
}
