mod assets;
pub mod components;
mod systems;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>().add_plugins(systems::plugin);
}

pub fn spawn_player_command(world: &mut World) {
    let _ = world.run_system_once(systems::spawn::spawn_player);
}
