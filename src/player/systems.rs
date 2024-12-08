use super::{
    assets::{blue_material, generate_pill_mesh},
    components::Player,
};
use crate::maze::{events::RecreateMazeEvent, MazeConfig};
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_config: Res<MazeConfig>,
) {
    let player_height = maze_config.height * 0.5;
    let player_radius = maze_config.hex_size * 0.5;

    commands.spawn((
        Name::new("Player"),
        Player,
        PbrBundle {
            mesh: meshes.add(generate_pill_mesh(player_radius, player_height / 2.)),
            material: materials.add(blue_material()),
            transform: Transform::from_xyz(0., player_height * 2., 0.),
            ..default()
        },
    ));
}
