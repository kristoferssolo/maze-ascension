use super::{
    components::MazeConfig,
    systems::{despawn::despawn_maze, respawn::respawn_maze, spawn::spawn_maze},
};
use bevy::{ecs::system::RunSystemOnce, prelude::*};

#[derive(Debug)]
pub struct SpawnMaze {
    pub floor: u8,
    pub config: MazeConfig,
}

#[derive(Debug)]
pub struct RespawnMaze {
    pub floor: u8,
    pub config: MazeConfig,
}

#[derive(Debug)]
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
    type Out = ();

    fn apply(self, world: &mut World) {
        let _ = world.run_system_once_with(spawn_maze, self);
    }
}

impl Command for RespawnMaze {
    type Out = ();

    fn apply(self, world: &mut World) {
        let _ = world.run_system_once_with(respawn_maze, self);
    }
}

impl Command for DespawnMaze {
    type Out = ();

    fn apply(self, world: &mut World) {
        let _ = world.run_system_once_with(despawn_maze, self);
    }
}
