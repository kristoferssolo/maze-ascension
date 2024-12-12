use super::spawn::spawn_floor;
use crate::maze::MazeConfig;
use bevy::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
) {
    spawn_floor(&mut commands, &mut meshes, &mut materials, &config);
}
