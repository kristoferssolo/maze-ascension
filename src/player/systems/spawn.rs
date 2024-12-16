use crate::{
    maze::{components::MazeConfig, GlobalMazeConfig},
    player::{
        assets::{blue_material, generate_pill_mesh},
        components::{CurrentPosition, Player},
    },
};
use bevy::prelude::*;

pub(super) fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) {
    let player_radius = global_config.hex_size * 0.5;
    let player_height = player_radius * 3.5;

    let y_offset = global_config.height / 2. + player_height / 1.3;

    let start_pos = maze_config.layout.hex_to_world_pos(maze_config.start_pos);

    commands.spawn((
        Name::new("Player"),
        Player,
        CurrentPosition(maze_config.start_pos),
        Mesh3d(meshes.add(generate_pill_mesh(player_radius, player_height / 2.))),
        MeshMaterial3d(materials.add(blue_material())),
        Transform::from_xyz(start_pos.x, y_offset, start_pos.y),
    ));
}
