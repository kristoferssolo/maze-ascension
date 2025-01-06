mod assets;
pub mod commands;
pub mod components;
mod systems;

use assets::PlayerAssets;
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::Player;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .load_resource::<PlayerAssets>()
        .add_plugins(systems::plugin);
}

pub fn spawn_player_command(world: &mut World) {
    let _ = world.run_system_once(systems::spawn::spawn_player);
}
