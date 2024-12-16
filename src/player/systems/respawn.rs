use super::{despawn::despawn_players, spawn::spawn_player};
use crate::{
    maze::{components::MazeConfig, GlobalMazeConfig},
    player::components::Player,
};
use bevy::prelude::*;

pub(super) fn respawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    query: &Query<Entity, With<Player>>,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) {
    despawn_players(commands, query);
    spawn_player(commands, meshes, materials, maze_config, global_config);
}
