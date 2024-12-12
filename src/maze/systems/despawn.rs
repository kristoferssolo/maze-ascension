use crate::maze::components::Floor;
use bevy::prelude::*;

pub(crate) fn despawn_floor(
    commands: &mut Commands,
    query: &Query<(Entity, &Floor)>,
    floor_num: u8,
) {
    for (entity, floor) in query.iter() {
        if floor.0 == floor_num {
            commands.entity(entity).despawn_recursive();
        }
    }
}
