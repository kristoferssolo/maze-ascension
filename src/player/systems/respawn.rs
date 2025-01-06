use crate::player::commands::{DespawnPlayer, SpawnPlayer};
use bevy::prelude::*;

pub fn respawn_player(mut commands: Commands) {
    commands.queue(DespawnPlayer);
    commands.queue(SpawnPlayer);
}
