use crate::{
    maze::MazeConfig,
    player::{components::Player, events::RespawnPlayer},
};
use bevy::prelude::*;

use super::{despawn::despawn_players, spawn::spawn_player};

pub(crate) fn respawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
    query: Query<Entity, With<Player>>,
    mut event_reader: EventReader<RespawnPlayer>,
) {
    for _ in event_reader.read() {
        despawn_players(&mut commands, &query);
        spawn_player(&mut commands, &mut meshes, &mut materials, &config);
    }
}
