use crate::player::events::{DespawnPlayer, RespawnPlayer, SpawnPlayer};
use bevy::prelude::*;

pub(super) fn respawn_player(_trigger: Trigger<RespawnPlayer>, mut commands: Commands) {
    commands.trigger(DespawnPlayer);
    commands.trigger(SpawnPlayer);
}
