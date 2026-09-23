mod pathfinder;
pub(crate) mod wall_jump;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((wall_jump::plugin, pathfinder::plugin));
}
