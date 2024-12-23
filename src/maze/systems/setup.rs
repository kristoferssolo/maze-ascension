use crate::maze::events::SpawnMaze;
use bevy::prelude::*;

pub(crate) fn setup(mut commands: Commands) {
    commands.trigger(SpawnMaze::default());
}
