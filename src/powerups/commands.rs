use bevy::{ecs::system::RunSystemOnce, prelude::*};

use super::systems::spawn_powerup;

#[derive(Debug, Reflect)]
pub struct SpawnPowerup;

impl Command for SpawnPowerup {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_once(spawn_powerup);
    }
}
