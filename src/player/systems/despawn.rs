use bevy::prelude::*;

use crate::player::components::Player;

pub(crate) fn despawn_players(commands: &mut Commands, query: &Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
