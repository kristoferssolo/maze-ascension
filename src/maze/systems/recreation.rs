use bevy::prelude::*;

use crate::maze::{components::Floor, events::RecreateMazeEvent, MazeConfig};

use super::{despawn::despawn_floor, spawn::spawn_floor};

pub(crate) fn recreate_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MazeConfig>,
    query: Query<(Entity, &Floor)>,
    mut event_reader: EventReader<RecreateMazeEvent>,
) {
    for event in event_reader.read() {
        despawn_floor(&mut commands, &query, event.floor);
        spawn_floor(&mut commands, &mut meshes, &mut materials, &config);
    }
}
