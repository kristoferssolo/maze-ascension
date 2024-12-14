pub mod event_handler;
pub mod setup;
pub mod spawn;
pub mod update;

use bevy::prelude::*;
use event_handler::handle_maze_events;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_maze_events);
}
