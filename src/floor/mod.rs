pub mod components;
pub mod events;
mod systems;

use bevy::prelude::*;
use events::TransitionFloor;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<TransitionFloor>()
        .add_plugins(systems::plugin);
}
