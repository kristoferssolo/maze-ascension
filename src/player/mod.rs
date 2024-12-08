mod assets;
mod components;
mod systems;

use bevy::prelude::*;
use components::Player;
use systems::spawn_player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .add_systems(Startup, spawn_player);
}
