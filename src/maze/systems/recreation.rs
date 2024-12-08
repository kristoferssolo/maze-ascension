use bevy::prelude::*;

use crate::maze::{
    components::MazeFloor, events::RecreateMazeEvent, resources::Layout, MazeConfig,
};

use super::setup::setup_maze;

pub(crate) fn handle_maze_recreation_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
    layout: Res<Layout>,
    query: Query<(Entity, &MazeFloor)>,
    mut event_reader: EventReader<RecreateMazeEvent>,
) {
    for event in event_reader.read() {
        despawn_floor(&mut commands, &query, event.floor);
        setup_maze(&mut commands, &mut meshes, &mut materials, &config, &layout);
    }
}

fn despawn_floor(commands: &mut Commands, query: &Query<(Entity, &MazeFloor)>, floor_num: u8) {
    for (entity, maze_floor) in query.iter() {
        if maze_floor.0 == floor_num {
            commands.entity(entity).despawn_recursive();
        }
    }
}
