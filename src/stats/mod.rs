pub mod components;
pub mod resources;
pub mod stats;
mod systems;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::Score;
use resources::{FloorTimer, GameTimer};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Score>()
        .init_resource::<GameTimer>()
        .init_resource::<FloorTimer>()
        .insert_resource(FloorTimer(Timer::from_seconds(0.0, TimerMode::Once)))
        .add_plugins(systems::plugin);
}

pub fn spawn_stats_command(world: &mut World) {
    let _ = world.run_system_once(systems::setup::setup);
}
