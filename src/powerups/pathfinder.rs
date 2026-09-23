use crate::{
    floor::components::{CurrentFloor, FloorYTarget},
    maze::{components::MazeConfig, GlobalMazeConfig},
    player::components::{CurrentPosition, MovementTarget, Player},
    screens::{GameplayElement, Screen},
    theme::{palette::rose_pine::RosePineDawn, prelude::ColorScheme},
    AppSystems,
};
use bevy::prelude::*;
use hexlab::Maze;

const COOLDOWN_SECONDS: f32 = 10.0;
const PREVIEW_SECONDS: f32 = 5.0;

#[derive(Debug, Default, Resource)]
pub(super) struct Pathfinder {
    cooldown: Option<Timer>,
}

impl Pathfinder {
    const fn is_ready(&self) -> bool {
        self.cooldown.is_none()
    }

    pub(super) fn cooldown_remaining_secs(&self) -> Option<f32> {
        self.cooldown.as_ref().map(Timer::remaining_secs)
    }

    fn consume(&mut self) {
        self.cooldown = Some(Timer::from_seconds(COOLDOWN_SECONDS, TimerMode::Once));
    }

    fn tick(&mut self, delta: std::time::Duration) {
        if self
            .cooldown
            .as_mut()
            .is_some_and(|timer| timer.tick(delta).is_finished())
        {
            self.cooldown = None;
        }
    }
}

#[derive(Component)]
struct RoutePreview(Timer);

#[derive(Resource)]
struct RouteVisuals {
    marker: Handle<Mesh>,
    route: Handle<StandardMaterial>,
    destination: Handle<StandardMaterial>,
}

impl FromWorld for RouteVisuals {
    fn from_world(world: &mut World) -> Self {
        let marker = world
            .get_resource_or_init::<Assets<Mesh>>()
            .add(Cylinder::new(1.0, 0.08).mesh().resolution(18));
        let mut materials = world.get_resource_or_init::<Assets<StandardMaterial>>();
        let route = materials.add(StandardMaterial {
            base_color: RosePineDawn::Gold.to_color().with_alpha(0.8),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });
        let destination = materials.add(StandardMaterial {
            base_color: RosePineDawn::Love.to_color().with_alpha(0.9),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });
        Self {
            marker,
            route,
            destination,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Pathfinder>()
        .init_resource::<RouteVisuals>()
        .add_systems(OnEnter(Screen::Title), reset)
        .add_systems(
            Update,
            tick_cooldown
                .in_set(AppSystems::TickTimers)
                .run_if(in_state(Screen::Gameplay)),
        )
        .add_systems(
            Update,
            (expire_routes, show_route)
                .chain()
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
}

fn reset(mut pathfinder: ResMut<Pathfinder>) {
    *pathfinder = Pathfinder::default();
}

fn tick_cooldown(time: Res<Time>, mut pathfinder: ResMut<Pathfinder>) {
    pathfinder.tick(time.delta());
}

fn expire_routes(
    mut commands: Commands,
    time: Res<Time>,
    current_floor: Query<Entity, With<CurrentFloor>>,
    mut previews: Query<(Entity, &ChildOf, &mut RoutePreview)>,
) {
    let current_floor = current_floor.single().ok();
    for (entity, parent, mut preview) in &mut previews {
        preview.0.tick(time.delta());
        if preview.0.is_finished() || current_floor != Some(parent.parent()) {
            commands.entity(entity).despawn();
        }
    }
}

fn show_route(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut pathfinder: ResMut<Pathfinder>,
    floor: Query<(Entity, &Maze, &MazeConfig, Option<&FloorYTarget>), With<CurrentFloor>>,
    player: Query<(&CurrentPosition, &MovementTarget), With<Player>>,
    config: Res<GlobalMazeConfig>,
    visuals: Res<RouteVisuals>,
) {
    if !input.just_pressed(KeyCode::KeyF) || !pathfinder.is_ready() {
        return;
    }
    let Ok((floor_entity, maze, maze_config, transition)) = floor.single() else {
        return;
    };
    if transition.is_some() {
        return;
    }
    let Ok((position, target)) = player.single() else {
        return;
    };
    if target.0.is_some() {
        return;
    }
    let Some(path) = maze.find_path(position.0, maze_config.end_pos) else {
        return;
    };
    if path.len() < 2 {
        return;
    }

    let height = config.height / 2.0 + 0.12;
    let marker_scale = config.hex_size * 0.25;
    commands.entity(floor_entity).with_children(|parent| {
        parent
            .spawn((
                Name::new("Route preview"),
                RoutePreview(Timer::from_seconds(PREVIEW_SECONDS, TimerMode::Once)),
                Transform::default(),
                Visibility::Visible,
                GameplayElement,
            ))
            .with_children(|markers| {
                for (step, hex) in path.iter().enumerate().skip(1) {
                    let pos = maze_config.layout.hex_to_world_pos(*hex);
                    let material = if step == path.len() - 1 {
                        visuals.destination.clone()
                    } else {
                        visuals.route.clone()
                    };
                    markers.spawn((
                        Name::new(format!("Route step {step}")),
                        Mesh3d(visuals.marker.clone()),
                        MeshMaterial3d(material),
                        Transform::from_xyz(pos.x, height, pos.y).with_scale(Vec3::new(
                            marker_scale,
                            1.0,
                            marker_scale,
                        )),
                    ));
                }
            });
    });
    pathfinder.consume();
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_ok, assert_some};
    use hexx::{EdgeDirection, Hex};

    fn route_app(open: bool) -> (App, Entity) {
        let mut app = App::new();
        app.init_resource::<GlobalMazeConfig>();
        app.init_resource::<Pathfinder>();
        app.init_resource::<RouteVisuals>();
        app.insert_resource(Time::<()>::default());
        let mut input = ButtonInput::default();
        input.press(KeyCode::KeyF);
        app.insert_resource(input);
        app.add_systems(Update, (expire_routes, show_route).chain());

        let direction = EdgeDirection::FLAT_SOUTH_EAST;
        let destination = Hex::ZERO.neighbor(direction);
        let mut maze = Maze::new();
        maze.insert(Hex::ZERO);
        maze.insert(destination);
        if open {
            assert_ok!(maze.remove_tile_wall(&Hex::ZERO, direction));
            assert_ok!(maze.remove_tile_wall(&destination, -direction));
        }
        let config = MazeConfig {
            start_pos: Hex::ZERO,
            end_pos: destination,
            ..default()
        };
        let floor = app.world_mut().spawn((CurrentFloor, maze, config)).id();
        app.world_mut().spawn((
            Player,
            CurrentPosition(Hex::ZERO),
            MovementTarget::default(),
        ));

        (app, floor)
    }

    #[test]
    fn route_preview_marks_path_to_exit_and_spends_cooldown() {
        let (mut app, floor) = route_app(true);

        app.update();

        let world = app.world_mut();
        let mut previews = world.query::<(Entity, &RoutePreview, &ChildOf, &Children)>();
        let (preview, _, parent, children) = assert_ok!(previews.single(world));
        assert_eq!(parent.parent(), floor);
        assert_eq!(children.len(), 1);
        let marker = assert_some!(world.get::<Transform>(children[0]));
        assert!(marker.translation.x > 0.0);
        assert!(!assert_some!(world.get_resource::<Pathfinder>()).is_ready());
        assert!(world.get_entity(preview).is_ok());
    }

    #[test]
    fn no_route_does_not_spend_cooldown() {
        let (mut app, _) = route_app(false);

        app.update();

        let world = app.world_mut();
        let mut previews = world.query::<&RoutePreview>();
        assert_eq!(previews.iter(world).count(), 0);
        assert!(assert_some!(world.get_resource::<Pathfinder>()).is_ready());
    }

    #[test]
    fn preview_expires_after_five_seconds() {
        let (mut app, _) = route_app(true);
        app.update();
        assert_some!(app.world_mut().get_resource_mut::<Time>())
            .advance_by(std::time::Duration::from_secs(5));

        app.update();

        let world = app.world_mut();
        let mut previews = world.query::<&RoutePreview>();
        assert_eq!(previews.iter(world).count(), 0);
    }

    #[test]
    fn preview_disappears_when_current_floor_changes() {
        let (mut app, floor) = route_app(true);
        app.update();
        assert_ok!(app.world_mut().get_entity_mut(floor)).remove::<CurrentFloor>();
        app.world_mut().spawn(CurrentFloor);

        app.update();

        let world = app.world_mut();
        let mut previews = world.query::<&RoutePreview>();
        assert_eq!(previews.iter(world).count(), 0);
    }
}
