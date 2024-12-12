use bevy::prelude::*;
use hexlab::HexMaze;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct Maze(pub(crate) HexMaze);

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct Floor(pub(crate) u8);

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct Tile;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct Wall;
