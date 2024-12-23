use bevy::prelude::*;

use crate::maze::components::MazeConfig;

#[derive(Debug, Reflect, Event)]
pub struct SpawnFloor {
    pub floor: u8,
    pub config: MazeConfig,
}

#[derive(Debug, Reflect, Event)]
pub struct RespawnFloor {
    pub floor: u8,
    pub config: MazeConfig,
}

#[derive(Debug, Reflect, Event)]
pub struct DespawnFloor {
    pub floor: u8,
}

#[derive(Debug, Reflect, Event, Default)]
pub enum TransitionFloor {
    #[default]
    Ascend,
    Descend,
}
