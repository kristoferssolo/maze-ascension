use bevy::prelude::*;

use super::components::MazeConfig;

#[derive(Debug, Reflect, Event, Default)]
pub struct SpawnMaze {
    pub floor: u8,
    pub config: MazeConfig,
}

#[derive(Debug, Reflect, Event)]
pub struct RespawnMaze {
    pub floor: u8,
    pub config: MazeConfig,
}

#[derive(Debug, Reflect, Event)]
pub struct DespawnMaze {
    pub floor: u8,
}
