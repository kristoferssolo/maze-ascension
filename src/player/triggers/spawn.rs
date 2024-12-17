use crate::{
    floor::components::CurrentFloor,
    maze::{components::MazeConfig, GlobalMazeConfig},
    player::{
        assets::{blue_material, generate_pill_mesh},
        components::{CurrentPosition, Player},
        events::SpawnPlayer,
    },
};
use bevy::prelude::*;

pub(super) fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_config_query: Query<&MazeConfig, With<CurrentFloor>>,
    global_config: Res<GlobalMazeConfig>,
) {
    let Ok(maze_config) = maze_config_query.get_single() else {
        return;
    };
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
