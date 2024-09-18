//! Spawn the main level.

use bevy::{ecs::world::Command, prelude::*};

use crate::tiles::player::SpawnPlayer;

use super::tile::{self, GridSettings, SpawnGrid};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GridSettings::default());
    app.add_plugins(tile::plugin);
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    SpawnGrid.apply(world);
    SpawnPlayer.apply(world);
}
