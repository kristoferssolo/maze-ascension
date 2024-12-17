use crate::maze::{components::MazeConfig, events::SpawnMaze};
use bevy::prelude::*;

pub(crate) fn setup(mut commands: Commands) {
    let config = MazeConfig::default();
    commands.trigger(SpawnMaze { floor: 1, config });
}
