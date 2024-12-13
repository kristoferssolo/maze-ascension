use super::spawn::spawn_floor;
use crate::maze::{components::MazeConfig, resources::GlobalMazeConfig};
use bevy::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    global_config: Res<GlobalMazeConfig>,
) {
    let maze_config = MazeConfig::default();
    spawn_floor(
        &mut commands,
        &mut meshes,
        &mut materials,
        &maze_config,
        &global_config,
    );
}
