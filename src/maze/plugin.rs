use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};

use super::{resources::Layout, systems, MazeConfig};

#[derive(Default)]
pub(crate) struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeConfig>().init_resource::<Layout>();
    }
}

impl Command for MazePlugin {
    fn apply(self, world: &mut World) {
        world.run_system_once(systems::setup::setup);
    }
}
