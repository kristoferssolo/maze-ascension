use super::{pathfinder::Pathfinder, wall_jump::WallJump};
use crate::{
    floor::components::{CurrentFloor, FloorYTarget},
    maze::{
        components::{HexMaze, MazeConfig},
        GlobalMazeConfig,
    },
    player::components::{CurrentPosition, MovementTarget, Player},
    screens::Screen,
    theme::{palette::rose_pine::RosePineDawn, prelude::ColorScheme},
    AppSystems,
};
use bevy::prelude::*;
use hexlab::{Maze, TilePosition};
use hexx::Hex;
use rand::{rngs::StdRng, RngExt, SeedableRng};

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
enum DropKind {
    WallJump,
    PathFinder,
}

#[derive(Component)]
struct PowerupDrop {
    kind: DropKind,
    hex: Hex,
}

#[derive(Resource)]
struct DropVisuals {
    mesh: Handle<Mesh>,
    wall_jump: Handle<StandardMaterial>,
    pathfinder: Handle<StandardMaterial>,
}

impl FromWorld for DropVisuals {
    fn from_world(world: &mut World) -> Self {
        let mesh = world
            .get_resource_or_init::<Assets<Mesh>>()
            .add(Sphere::new(1.0));
        let mut materials = world.get_resource_or_init::<Assets<StandardMaterial>>();
        let wall_jump = materials.add(RosePineDawn::Pine.to_color());
        let pathfinder = materials.add(RosePineDawn::Gold.to_color());
        Self {
            mesh,
            wall_jump,
            pathfinder,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DropVisuals>().add_systems(
        Update,
        (spawn_drops, collect_drops)
            .chain()
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

fn spawn_drops(
    mut commands: Commands,
    floors: Query<(Entity, &Maze, &MazeConfig), (With<HexMaze>, Added<Maze>)>,
    visuals: Res<DropVisuals>,
    global_config: Res<GlobalMazeConfig>,
) {
    for (floor, maze, config) in &floors {
        let mut positions = maze
            .values()
            .map(|tile| tile.pos())
            .filter(|hex| *hex != config.start_pos && *hex != config.end_pos)
            .collect::<Vec<_>>();
        positions.sort_by_key(|hex| (hex.x, hex.y));
        // The maze seed makes drop placement reproducible without coupling it to generation.
        let mut rng = StdRng::seed_from_u64(config.seed ^ 0x6d31_5a7b_8e29_c04f);
        let count = positions.len().min(rng.random_range(1..=2));
        for _ in 0..count {
            let index = rng.random_range(0..positions.len());
            let hex = positions.swap_remove(index);
            let kind = if rng.random_bool(0.5) {
                DropKind::WallJump
            } else {
                DropKind::PathFinder
            };
            let material = match kind {
                DropKind::WallJump => &visuals.wall_jump,
                DropKind::PathFinder => &visuals.pathfinder,
            };
            let pos = config.layout.hex_to_world_pos(hex);
            commands.entity(floor).with_children(|parent| {
                parent.spawn((
                    Name::new(format!("{kind:?} drop")),
                    PowerupDrop { kind, hex },
                    Mesh3d(visuals.mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(pos.x, global_config.height / 2.0 + 1.5, pos.y)
                        .with_scale(Vec3::splat(global_config.hex_size * 0.22)),
                ));
            });
        }
    }
}

fn collect_drops(
    mut commands: Commands,
    floor: Query<Entity, (With<CurrentFloor>, Without<FloorYTarget>)>,
    player: Query<(&CurrentPosition, &MovementTarget), With<Player>>,
    drops: Query<(Entity, &PowerupDrop, &ChildOf)>,
    mut wall_jump: ResMut<WallJump>,
    mut pathfinder: ResMut<Pathfinder>,
) {
    let (Ok(floor), Ok((position, movement))) = (floor.single(), player.single()) else {
        return;
    };
    if movement.0.is_some() {
        return;
    }
    for (entity, drop, parent) in &drops {
        if parent.parent() != floor || drop.hex != position.0 {
            continue;
        }
        match drop.kind {
            DropKind::WallJump => wall_jump.grant(),
            DropKind::PathFinder => pathfinder.grant(),
        }
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_some;

    #[test]
    fn collects_only_a_drop_on_the_current_floor() {
        let mut app = App::new();
        app.init_resource::<WallJump>();
        app.init_resource::<Pathfinder>();
        app.add_systems(Update, collect_drops);
        let current = app.world_mut().spawn(CurrentFloor).id();
        let other = app.world_mut().spawn_empty().id();
        app.world_mut().spawn((Player, CurrentPosition(Hex::ZERO)));
        let picked = app
            .world_mut()
            .spawn((
                PowerupDrop {
                    kind: DropKind::WallJump,
                    hex: Hex::ZERO,
                },
                ChildOf(current),
            ))
            .id();
        let unpicked = app
            .world_mut()
            .spawn((
                PowerupDrop {
                    kind: DropKind::PathFinder,
                    hex: Hex::ZERO,
                },
                ChildOf(other),
            ))
            .id();

        app.update();

        assert_eq!(
            assert_some!(app.world().get_resource::<WallJump>()).charges(),
            1
        );
        assert_eq!(
            assert_some!(app.world().get_resource::<Pathfinder>()).charges(),
            0
        );
        assert!(app.world().get_entity(picked).is_err());
        assert!(app.world().get_entity(unpicked).is_ok());
    }

    #[test]
    fn drops_avoid_start_and_exit() {
        let mut app = App::new();
        app.init_resource::<DropVisuals>();
        app.init_resource::<GlobalMazeConfig>();
        app.add_systems(Update, spawn_drops);
        let mut maze = Maze::new();
        let middle = Hex::new(1, 0);
        let end = Hex::new(2, 0);
        for hex in [Hex::ZERO, middle, end] {
            maze.insert(hex);
        }
        let config = MazeConfig {
            start_pos: Hex::ZERO,
            end_pos: end,
            seed: 42,
            ..default()
        };
        let floor = app.world_mut().spawn((HexMaze, maze, config)).id();

        app.update();

        let world = app.world_mut();
        let mut drops = world.query::<(&PowerupDrop, &ChildOf)>();
        let found = drops.iter(world).collect::<Vec<_>>();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].0.hex, middle);
        assert_eq!(found[0].1.parent(), floor);
    }
}
