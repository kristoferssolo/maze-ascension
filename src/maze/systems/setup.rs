use crate::maze::events::SpawnMaze;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.trigger(SpawnMaze::default());
}
