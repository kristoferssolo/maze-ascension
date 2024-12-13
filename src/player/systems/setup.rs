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
    let maze_config = maze_config_query.single();
    spawn_player(
        &mut commands,
        &mut meshes,
        &mut materials,
        &maze_config,
        &global_config,
    );
}
