use bevy::prelude::*;

use crate::maze::MazeConfig;

use super::spawn::spawn_player;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
) {
    spawn_player(&mut commands, &mut meshes, &mut materials, &config);
}
