use crate::{
    floor::components::{CurrentFloor, Floor},
    maze::{
        assets::MazeAssets,
        components::Maze,
        events::SpawnMaze,
        triggers::{
            common::generate_maze,
            spawn::{spawn_maze_tiles, FLOOR_Y_OFFSET},
        },
        GlobalMazeConfig,
    },
};
use bevy::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_query: Query<(Entity, &Floor, &Maze)>,
    global_config: Res<GlobalMazeConfig>,
) {
    let SpawnMaze { floor, config } = SpawnMaze::default();
    if maze_query.iter().any(|(_, f, _)| f.0 == floor) {
        warn!("Floor {} already exists, skipping creation", floor);
        return;
    }

    let maze = generate_maze(&config).expect("Failed to generate maze during spawn");
    let y_offset = (floor - 1) * FLOOR_Y_OFFSET;

    let entity = commands
        .spawn((
            Name::new(format!("Floor {}", floor)),
            Maze(maze.clone()),
            Floor(floor),
            CurrentFloor,
            config.clone(),
            Transform::from_translation(Vec3::ZERO.with_y(y_offset as f32)),
            Visibility::Visible,
        ))
        .id();

    let assets = MazeAssets::new(&mut meshes, &mut materials, &global_config);
    spawn_maze_tiles(
        &mut commands,
        entity,
        &maze,
        &assets,
        &config,
        &global_config,
    );
}
