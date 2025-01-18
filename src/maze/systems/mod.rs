pub mod common;
pub mod despawn;
pub mod respawn;
pub mod spawn;
mod toogle_pause;

use bevy::prelude::*;
use toogle_pause::toggle_walls;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, toggle_walls.run_if(state_changed::<Screen>));
}
