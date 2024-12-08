use bevy::prelude::*;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct MazeFloor(pub(crate) u8);

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct MazeTile;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct MazeWall;
