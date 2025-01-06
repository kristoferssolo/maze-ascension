use bevy::{ecs::system::RunSystemOnce, prelude::*};

use super::systems::{despawn::despawn_players, respawn::respawn_player, spawn::spawn_player};

#[derive(Debug, Reflect)]
pub struct SpawnPlayer;

#[derive(Debug, Reflect)]
pub struct RespawnPlayer;

#[derive(Debug, Reflect)]
pub struct DespawnPlayer;

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_once(spawn_player);
    }
}

impl Command for RespawnPlayer {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_once(respawn_player);
    }
}

impl Command for DespawnPlayer {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_once(despawn_players);
    }
}
