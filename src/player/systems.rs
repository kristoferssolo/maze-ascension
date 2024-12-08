use super::components::Player;
use crate::maze::{events::RecreateMazeEvent, MazeConfig};
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_config: Res<MazeConfig>,
) {
}

pub fn respawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut recreation_events: EventReader<RecreateMazeEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_config: Res<MazeConfig>,
) {
}
