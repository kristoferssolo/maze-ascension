use crate::player::events::SpawnPlayer;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.trigger(SpawnPlayer);
}
