//! Maze spawning and rendering functionality.
//!
//! Module handles the creation and visualization of hexagonal mazes.

use super::common::generate_maze;
use crate::{
    constants::FLOOR_Y_OFFSET,
    floor::{
        components::{CurrentFloor, Floor},
        events::TransitionFloor,
    },
    maze::{
        assets::MazeAssets,
        commands::SpawnMaze,
        components::{HexMaze, MazeConfig, Tile, Wall},
        resources::GlobalMazeConfig,
    },
    screens::GameplayElement,
    theme::palette::rose_pine::RosePineDawn,
};

use bevy::prelude::*;
use hexlab::prelude::{Tile as HexTile, *};
use hexx::{EdgeDirection, Hex, HexLayout, HexOrientation};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_6};

/// Spawns a new maze for the specified floor on [`SpawnMaze`] event.
pub fn spawn_maze(
    In(SpawnMaze { floor, config }): In<SpawnMaze>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_query: Query<(Entity, &Floor, &Maze)>,
    global_config: Res<GlobalMazeConfig>,
    mut event_writer: MessageWriter<TransitionFloor>,
) {
    if maze_query.iter().any(|(_, f, _)| f.0 == floor) {
        info!("Floor {} already exists, skipping creation", floor);
        return;
    }

    let maze = match generate_maze(&config) {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to generate maze for floor {floor}: {:?}", e);
            return;
        }
    };

    // Calculate vertical offset based on floor number
    let y_offset = match floor {
        1 => 0,              // Ground/Initial floor (floor 1) is at y=0
        _ => FLOOR_Y_OFFSET, // Other floors are offset vertically
    } as f32;

    let entity = commands
        .spawn((
            Name::new(format!("Floor {}", floor)),
            HexMaze,
            maze.clone(),
            Floor(floor),
            config.clone(),
            Transform::from_translation(Vec3::ZERO.with_y(y_offset)),
            Visibility::Visible,
            GameplayElement,
        ))
        .insert_if(CurrentFloor, || floor == 1) // Only floor 1 gets CurrentFloor
        .id();

    let assets = MazeAssets::new(&mut meshes, &mut materials, &global_config);

    spawn_maze_tiles(commands, entity, &maze, &assets, &config, &global_config);

    // TODO: find a better way to handle double event indirection
    if floor != 1 {
        event_writer.write(TransitionFloor::Ascend);
    }
}

/// Spawns all tiles for a maze as children of the parent maze entity
pub fn spawn_maze_tiles<'world, 'state>(
    mut commands: Commands<'world, 'state>,
    parent_entity: Entity,
    maze: &Maze,
    assets: &MazeAssets,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) -> Commands<'world, 'state> {
    commands.entity(parent_entity).with_children(|parent| {
        for tile in maze.values() {
            spawn_single_hex_tile(parent, assets, tile, maze_config, global_config);
        }
    });
    commands
}

/// Spawns a single hexagonal tile with appropriate transforms and materials
pub(super) fn spawn_single_hex_tile(
    parent: &mut ChildSpawnerCommands,
    assets: &MazeAssets,
    tile: &HexTile,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) {
    let world_pos = maze_config.layout.hex_to_world_pos(tile.pos());
    let world_pos = Vec3::new(world_pos.x, 0., world_pos.y);
    let rotation = match maze_config.layout.orientation {
        HexOrientation::Pointy => Quat::from_rotation_y(0.0),
        HexOrientation::Flat => Quat::from_rotation_y(FRAC_PI_6), // 30 degrees rotation
    };

    // Select material based on tile position: start, end, or default
    let material = match tile.pos() {
        pos if pos == maze_config.start_pos => assets
            .custom_materials
            .get(&RosePineDawn::Pine)
            .cloned()
            .unwrap_or_default(),
        pos if pos == maze_config.end_pos => assets
            .custom_materials
            .get(&RosePineDawn::Love)
            .cloned()
            .unwrap_or_default(),
        _ => assets.hex_material.clone(),
    };

    parent
        .spawn((
            Name::new(format!("Hex {}", tile)),
            Tile,
            Mesh3d(assets.hex_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos).with_rotation(rotation),
        ))
        .with_children(|parent| {
            spawn_walls(
                parent,
                assets,
                tile.walls(),
                &maze_config.layout,
                rotation,
                global_config,
            );
        });
}

/// Spawns walls around a hexagonal tile based on the walls configuration
fn spawn_walls(
    parent: &mut ChildSpawnerCommands,
    assets: &MazeAssets,
    walls: &Walls,
    layout: &HexLayout,
    tile_rotation: Quat,
    global_config: &GlobalMazeConfig,
) {
    // Base rotation for wall alignment (90 degrees counter-clockwise)
    let z_rotation = Quat::from_rotation_z(-FRAC_PI_2);
    let y_offset = global_config.height / 2.;

    let center = layout.hex_to_world_pos(Hex::ZERO);
    for direction in EdgeDirection::ALL_DIRECTIONS {
        if !walls.contains(direction) {
            continue;
        }

        let neighbor = layout.hex_to_world_pos(Hex::ZERO.neighbor(direction));
        let offset = (neighbor - center).normalize() * global_config.wall_offset();
        // Walls are children of the rotated tile, so undo that rotation here.
        let local_offset = tile_rotation.inverse() * Vec3::new(offset.x, 0.0, offset.y);
        let wall_angle = local_offset.z.atan2(local_offset.x);
        let pos = local_offset.with_y(y_offset);

        // 1. Rotate around x-axis to align wall with angle
        // 2. Add FRAC_PI_2 (90) to make wall perpendicular to angle
        let x_rotation = Quat::from_rotation_x(wall_angle + FRAC_PI_2);
        let final_rotation = z_rotation * x_rotation;

        spawn_single_wall(parent, assets, final_rotation, pos);
    }
}

/// Spawns a single wall segment with the specified rotation and position
fn spawn_single_wall(
    parent: &mut ChildSpawnerCommands,
    assets: &MazeAssets,
    rotation: Quat,
    offset: Vec3,
) {
    parent.spawn((
        Name::new("Wall"),
        Wall,
        Mesh3d(assets.wall_mesh.clone()),
        MeshMaterial3d(assets.wall_material.clone()),
        Transform::from_translation(offset).with_rotation(rotation),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::world::CommandQueue;
    use claims::assert_ok;

    #[test]
    fn wall_position_faces_its_hex_neighbor() {
        let assets = MazeAssets {
            hex_mesh: Handle::default(),
            wall_mesh: Handle::default(),
            hex_material: Handle::default(),
            wall_material: Handle::default(),
            custom_materials: Default::default(),
        };
        for orientation in [HexOrientation::Flat, HexOrientation::Pointy] {
            for direction in EdgeDirection::ALL_DIRECTIONS {
                let mut world = World::new();
                let mut queue = CommandQueue::default();
                let mut walls = Walls::empty();
                walls.insert(direction);
                let mut config = MazeConfig::default();
                config.layout.orientation = orientation;
                let tile_rotation = match orientation {
                    HexOrientation::Pointy => Quat::IDENTITY,
                    HexOrientation::Flat => Quat::from_rotation_y(FRAC_PI_6),
                };

                Commands::new(&mut queue, &world)
                    .spawn_empty()
                    .with_children(|parent| {
                        spawn_walls(
                            parent,
                            &assets,
                            &walls,
                            &config.layout,
                            tile_rotation,
                            &GlobalMazeConfig::default(),
                        );
                    });
                queue.apply(&mut world);

                let mut query = world.query_filtered::<&Transform, With<Wall>>();
                let wall = assert_ok!(query.single(&world));
                let world_offset = tile_rotation * wall.translation;
                let center = config.layout.hex_to_world_pos(Hex::ZERO);
                let neighbor = config
                    .layout
                    .hex_to_world_pos(Hex::ZERO.neighbor(direction));
                let wall_direction = Vec2::new(world_offset.x, world_offset.z).normalize();
                assert!(
                    wall_direction.dot((neighbor - center).normalize()) > 0.99,
                    "{orientation:?} {direction:?}",
                );
            }
        }
    }
}
