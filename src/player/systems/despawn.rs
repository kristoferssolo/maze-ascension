use crate::player::components::Player;
use bevy::prelude::*;

pub fn despawn_players(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
