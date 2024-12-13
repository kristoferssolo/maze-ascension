use bevy::prelude::*;

use crate::{
    floor::components::Floor,
    maze::{components::MazeConfig, events::RecreateMazeEvent, GlobalMazeConfig},
};

use super::{despawn::despawn_floor, spawn::spawn_floor};

pub(crate) fn recreate_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Floor)>,
    mut event_reader: EventReader<RecreateMazeEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    global_config: Res<GlobalMazeConfig>,
) {
    let maze_config = MazeConfig::default();
    for event in event_reader.read() {
        despawn_floor(&mut commands, &query, event.floor);
        spawn_floor(
            &mut commands,
            &mut meshes,
            &mut materials,
            &maze_config,
            &global_config,
        );
    }
}
