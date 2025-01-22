pub mod commands;
pub mod components;
mod systems;

use bevy::prelude::*;
use commands::SpawnPowerup;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(systems::plugin);
}

pub fn spawn_powerup_command(world: &mut World) {
    SpawnPowerup.apply(world);
}
