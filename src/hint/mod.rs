pub mod assets;
pub mod components;
mod systems;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::IdleTimer;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IdleTimer>()
        .add_plugins(systems::plugin);
}

pub fn spawn_hint_command(world: &mut World) {
    let _ = world.run_system_once(systems::spawn::spawn_hints);
}
