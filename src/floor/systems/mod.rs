mod despawn;
mod spawn;

use crate::maze::MazePluginLoaded;
use bevy::prelude::*;
use despawn::despawn_level;
use spawn::spawn_floor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn_floor, despawn_level).run_if(resource_exists::<MazePluginLoaded>),
    );
}
