use super::{spawn::spawn_floor, update::update_floor};
use crate::{
    floor::components::Floor,
    maze::{components::Maze, events::MazeEvent, GlobalMazeConfig},
};
use bevy::prelude::*;

pub(crate) fn handle_maze_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_reader: EventReader<MazeEvent>,
    mut maze_query: Query<(Entity, &Floor, &mut Maze)>,
    global_config: Res<GlobalMazeConfig>,
) {
    for event in event_reader.read() {
        match event {
            MazeEvent::Create { floor, config } => {
                if maze_query.iter().any(|(_, f, _)| f.0 == *floor) {
                    warn!("Floor {} already exists, skipping creation", floor);
                    continue;
                }
                spawn_floor(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    *floor,
                    config,
                    &global_config,
                );
            }
            MazeEvent::Recreate { floor, config } => {
                let result = update_floor(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut maze_query,
                    *floor,
                    config,
                    &global_config,
                );
                if let Err(e) = result {
                    warn!("Failed to update floor {}: {}", floor, e);
                }
            }
            MazeEvent::Remove { floor } => {
                match maze_query.iter().find(|(_, f, _)| f.0 == *floor) {
                    Some((entity, _, _)) => commands.entity(entity).despawn_recursive(),
                    _ => warn!("Floor {} not found for removal", floor),
                }
            }
        }
    }
}
