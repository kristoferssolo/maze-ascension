mod despawn;
mod event_handler;
mod input;
mod movement;
mod respawn;
pub mod setup;
mod spawn;

use bevy::prelude::*;
use event_handler::handle_player_events;
use input::player_input;
use movement::player_movement;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_input,
            player_movement.after(player_input),
            handle_player_events,
        ),
    );
}
