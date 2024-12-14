use crate::{
    floor::components::CurrentFloor,
    maze::{components::MazeConfig, GlobalMazeConfig},
    player::{components::Player, events::PlayerEvent},
};
use bevy::prelude::*;

use super::{despawn::despawn_players, respawn::respawn_player, spawn::spawn_player};

pub(crate) fn handle_player_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_reader: EventReader<PlayerEvent>,
    maze_config_query: Query<&MazeConfig, With<CurrentFloor>>,
    player_query: Query<Entity, With<Player>>,
    global_config: Res<GlobalMazeConfig>,
) {
    for event in event_reader.read() {
        match event {
            PlayerEvent::Spawn => {
                let Ok(maze_config) = maze_config_query.get_single() else {
                    warn!(
                        "Failed to get maze configuration for current floor - cannot spawn player"
                    );
                    return;
                };
                spawn_player(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    maze_config,
                    &global_config,
                );
            }
            PlayerEvent::Respawn => {
                let Ok(maze_config) = maze_config_query.get_single() else {
                    warn!("Failed to get maze configuration for current floor - cannot respawn player");
                    return;
                };
                respawn_player(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &player_query,
                    maze_config,
                    &global_config,
                );
            }
            PlayerEvent::Despawn => despawn_players(&mut commands, &player_query),
        }
    }
}
