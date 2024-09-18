use bevy::prelude::*;
pub mod direction;
pub mod grid;
pub mod tile;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((direction::plugin, tile::plugin, grid::plugin));
}
