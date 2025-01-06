use super::{
    components::MazeConfig,
    systems::{despawn::despawn_maze, respawn::respawn_maze, spawn::spawn_maze},
};
use bevy::{ecs::system::RunSystemOnce, prelude::*};

#[derive(Debug, Reflect)]
pub struct SpawnMaze {
    pub floor: u8,
    pub config: MazeConfig,
}

#[derive(Debug, Reflect)]
pub struct RespawnMaze {
    pub floor: u8,
    pub config: MazeConfig,
}

#[derive(Debug, Reflect)]
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

impl Command for SpawnMaze {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_once_with(self, spawn_maze);
    }
}

impl Command for RespawnMaze {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_once_with(self, respawn_maze);
    }
}

impl Command for DespawnMaze {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_once_with(self, despawn_maze);
    }
}
