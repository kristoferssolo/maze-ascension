mod components;
mod systems;

use bevy::prelude::*;
use components::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>().add_plugins(());
}
