pub mod components;
pub mod events;
pub mod resources;
mod systems;

use bevy::prelude::*;
use events::TransitionFloor;
use resources::HighestFloor;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<TransitionFloor>()
        .insert_resource(HighestFloor(1))
        .add_plugins(systems::plugin);
}
