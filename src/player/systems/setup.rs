use bevy::prelude::*;

use crate::{
    floor::components::CurrentFloor,
    maze::{components::MazeConfig, GlobalMazeConfig},
};

use super::spawn::spawn_player;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_config_query: Query<&MazeConfig, With<CurrentFloor>>,
    global_config: Res<GlobalMazeConfig>,
) {
    let Ok(maze_config) = maze_config_query.get_single() else {
        error!("Failed to get maze configuration for current floor - cannot spawn player");
        return;
    };

    spawn_player(
        &mut commands,
        &mut meshes,
        &mut materials,
        &maze_config,
        &global_config,
    );
}
