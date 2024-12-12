use bevy::prelude::*;

use crate::maze::{components::Floor, events::RecreateMazeEvent, MazeConfig};

use super::setup::setup_maze;

pub(crate) fn handle_maze_recreation_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
    query: Query<(Entity, &Floor)>,
    mut event_reader: EventReader<RecreateMazeEvent>,
) {
    for event in event_reader.read() {
        despawn_floor(&mut commands, &query, event.floor);
        setup_maze(&mut commands, &mut meshes, &mut materials, &config);
    }
}

fn despawn_floor(commands: &mut Commands, query: &Query<(Entity, &Floor)>, floor_num: u8) {
    for (entity, floor) in query.iter() {
        if floor.0 == floor_num {
            commands.entity(entity).despawn_recursive();
        }
    }
}
