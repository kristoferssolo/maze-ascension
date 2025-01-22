use activate::{handle_activation, monitor};
use bevy::prelude::*;

use super::components::{Kind, Powerup};
mod activate;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (handle_activation, monitor));
}

pub fn spawn_powerup(mut commands: Commands) {
    commands.spawn((Powerup, Kind::WallJump));
    commands.spawn((Powerup, Kind::PathFinder));
}
