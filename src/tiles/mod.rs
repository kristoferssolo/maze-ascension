use bevy::prelude::*;
pub mod level;
pub mod player;
pub mod tile;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, level::plugin));
}
