use crate::{
    floor::components::CurrentFloor,
    maze::{components::MazeConfig, GlobalMazeConfig},
    player::{components::Player, events::RespawnPlayer},
};
use bevy::prelude::*;

use super::{despawn::despawn_players, spawn::spawn_player};

pub(crate) fn respawn_player(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
    maze_config_query: Query<&MazeConfig, With<CurrentFloor>>,
    mut event_reader: EventReader<RespawnPlayer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    global_config: Res<GlobalMazeConfig>,
) {
    let maze_config = maze_config_query.single();
    for _ in event_reader.read() {
        despawn_players(&mut commands, &query);
        spawn_player(
            &mut commands,
            &mut meshes,
            &mut materials,
            &maze_config,
            &global_config,
        );
    }
}
