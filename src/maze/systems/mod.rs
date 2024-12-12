pub mod despawn;
pub mod recreation;
pub mod setup;
pub mod spawn;

use bevy::prelude::*;
use recreation::recreate_maze;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, recreate_maze);
}
