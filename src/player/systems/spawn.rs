use crate::{
    maze::MazeConfig,
    player::{
        assets::{blue_material, generate_pill_mesh},
        components::{CurrentPosition, Player},
    },
};
use bevy::prelude::*;
use hexx::Hex;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_config: Res<MazeConfig>,
) {
    let player_height = maze_config.height * 0.5;
    let player_radius = maze_config.hex_size * 0.5;

    let start_hex = Hex::new(1, 1);
    let start_pos = maze_config.layout.hex_to_world_pos(start_hex);

    commands.spawn((
        Name::new("Player"),
        Player,
        CurrentPosition(start_hex),
        Mesh3d(meshes.add(generate_pill_mesh(player_radius, player_height / 2.))),
        MeshMaterial3d(materials.add(blue_material())),
        Transform::from_xyz(start_pos.x, player_height * 2., start_pos.y),
    ));
}
