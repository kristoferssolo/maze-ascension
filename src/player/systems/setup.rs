use crate::player::events::SpawnPlayer;
use bevy::prelude::*;

pub(crate) fn setup(mut commands: Commands) {
    commands.trigger(SpawnPlayer);
}
