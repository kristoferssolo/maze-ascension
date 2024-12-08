use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};

use super::{
    events::RecreateMazeEvent,
    systems::{self, recreation::handle_maze_recreation_event},
    MazeConfig, MazePluginLoaded,
};

#[derive(Default)]
pub(crate) struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeConfig>()
            .add_event::<RecreateMazeEvent>()
            .add_systems(Update, handle_maze_recreation_event);
    }
}

impl Command for MazePlugin {
    fn apply(self, world: &mut World) {
        world.insert_resource(MazePluginLoaded);
        world.run_system_once(systems::setup::setup);
    }
}
