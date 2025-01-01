pub mod common;
mod despawn;
mod respawn;
pub mod spawn;

use bevy::prelude::*;
use despawn::despawn_maze;
use respawn::respawn_maze;
use spawn::spawn_maze;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_maze)
        .add_observer(respawn_maze)
        .add_observer(despawn_maze);
}
