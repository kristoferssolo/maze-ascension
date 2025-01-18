mod despawn;
mod hide;
mod movement;
mod spawn;

use crate::screens::Screen;
use bevy::prelude::*;
use despawn::despawn_floor;
use hide::hide_upper_floors;
use movement::{handle_floor_transition_events, move_floors};
use spawn::spawn_floor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            spawn_floor,
            despawn_floor,
            handle_floor_transition_events,
            move_floors,
            hide_upper_floors,
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
}
