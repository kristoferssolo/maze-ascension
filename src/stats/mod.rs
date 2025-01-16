pub mod components;
pub mod container;
pub mod resources;
mod systems;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::Score;
use resources::{FloorTimer, TotalTimer};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Score>()
        .init_resource::<TotalTimer>()
        .init_resource::<FloorTimer>()
        .add_plugins(systems::plugin);
}

pub fn spawn_stats_command(world: &mut World) {
    let _ = world.run_system_once(systems::setup::setup);
}
