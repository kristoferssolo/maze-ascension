pub mod components;
pub mod events;
mod systems;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(systems::plugin);
}
