use crate::player::{components::Player, events::DespawnPlayer};
use bevy::prelude::*;

pub(super) fn despawn_players(
    _trigger: Trigger<DespawnPlayer>,
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
