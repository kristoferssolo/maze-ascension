//! Spawn the main level.

use bevy::{ecs::world::Command, prelude::*};

use super::grid::SpawnGrid;

pub(super) fn plugin(_app: &mut App) {}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    SpawnGrid.apply(world);
}
