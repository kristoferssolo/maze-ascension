use bevy::prelude::*;
pub mod direction;
pub mod grid;
pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(level::plugin);
}
