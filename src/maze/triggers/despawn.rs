use crate::{floor::components::Floor, maze::events::DespawnMaze};
use bevy::prelude::*;

pub(super) fn despawn_maze(
    trigger: Trigger<DespawnMaze>,
    mut commands: Commands,
    query: Query<(Entity, &Floor)>,
) {
    let DespawnMaze { floor } = trigger.event();
    match query.iter().find(|(_, f)| f.0 == *floor) {
        Some((entity, _)) => commands.entity(entity).despawn_recursive(),
        _ => warn!("Floor {} not found for removal", floor),
    }
}
