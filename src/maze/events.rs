use super::components::MazeConfig;
use bevy::prelude::*;

#[derive(Debug, Reflect, Event)]
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

impl Default for SpawnMaze {
    fn default() -> Self {
        Self {
            floor: 1,
            config: MazeConfig::default(),
        }
    }
}
