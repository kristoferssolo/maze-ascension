mod despawn;
mod fog;
mod movement;
mod spawn;

use crate::maze::MazePluginLoaded;
use bevy::prelude::*;
use despawn::despawn_floor;
use fog::setup_camera_fog;
use movement::{handle_floor_transition_events, move_floors};
use spawn::spawn_floor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera_fog);
    app.add_systems(
        Update,
        (
            spawn_floor,
            despawn_floor,
            handle_floor_transition_events,
            move_floors.after(handle_floor_transition_events),
        )
            .run_if(resource_exists::<MazePluginLoaded>),
    );
}
