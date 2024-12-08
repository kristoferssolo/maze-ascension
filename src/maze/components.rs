use bevy::prelude::*;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct MazeWall;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct MazeTile;
