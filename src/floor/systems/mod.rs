mod despawn;
mod movement;
mod spawn;

use crate::maze::MazePluginLoaded;
use bevy::prelude::*;
use despawn::despawn_floor;
use movement::floor_movement;
use spawn::spawn_floor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn_floor, despawn_floor, floor_movement)
            .chain()
            .run_if(resource_exists::<MazePluginLoaded>),
    );
}
