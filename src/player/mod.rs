mod assets;
pub mod components;
pub mod events;
mod systems;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::Player;
use events::RespawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .add_event::<RespawnPlayer>()
        .add_plugins(systems::plugin);
}

pub fn spawn_player_command(world: &mut World) {
    let _ = world.run_system_once(systems::setup::setup);
}
