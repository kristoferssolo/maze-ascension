mod assets;
pub mod components;
pub mod events;
mod systems;
mod triggers;

use assets::PlayerAssets;
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use components::Player;
use events::{DespawnPlayer, RespawnPlayer, SpawnPlayer};

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .load_resource::<PlayerAssets>()
        .add_event::<SpawnPlayer>()
        .add_event::<RespawnPlayer>()
        .add_event::<DespawnPlayer>()
        .add_plugins((triggers::plugin, systems::plugin));
}

pub fn spawn_player_command(world: &mut World) {
    let _ = world.run_system_once(systems::setup::setup);
}
