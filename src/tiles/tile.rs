use bevy::{
    color::palettes::css::GRAY,
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};
use hexx::{EdgeDirection, Hex, HexLayout, HexOrientation};
use rand::{seq::IteratorRandom, thread_rng};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, generate_maze);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HexDirection {
    Top,
    TopRight,
    BottomRight,
    Bottom,
    BottomLeft,
    TopLeft,
}

#[derive(Debug)]
pub struct SpawnGrid;

impl Command for SpawnGrid {
    fn apply(self, world: &mut World) {
        world.run_system_once(setup_hex_grid);
    }
}

impl HexDirection {
    pub fn to_hexx_direction(self) -> EdgeDirection {
        self.into()
    }

    pub const ALL: [HexDirection; 6] = [
        Self::Top,
        Self::TopRight,
        Self::BottomRight,
        Self::Bottom,
        Self::BottomLeft,
        Self::TopLeft,
    ];

    pub fn opposite(&self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::TopRight => Self::BottomLeft,
            Self::BottomRight => Self::TopLeft,
            Self::Bottom => Self::Top,
            Self::BottomLeft => Self::TopRight,
            Self::TopLeft => Self::BottomRight,
        }
    }
}

impl From<HexDirection> for EdgeDirection {
    fn from(value: HexDirection) -> Self {
        match value {
            HexDirection::Top => Self::FLAT_NORTH,
            HexDirection::TopRight => Self::FLAT_NORTH_EAST,
            HexDirection::BottomRight => Self::FLAT_SOUTH_EAST,
            HexDirection::Bottom => Self::FLAT_SOUTH,
            HexDirection::BottomLeft => Self::FLAT_SOUTH_WEST,
            HexDirection::TopLeft => Self::FLAT_NORTH_WEST,
        }
    }
}

#[derive(Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct GridSettings {
    pub radius: u32,
    pub hex_size: Vec2,
}

#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct Tile {
    pub position: Hex,
    pub walls: HashMap<HexDirection, bool>,
}

impl Tile {
    pub fn new(position: Hex) -> Self {
        let mut walls = HashMap::new();
        for direction in HexDirection::ALL {
            walls.insert(direction, true);
        }
        Self { position, walls }
    }

    pub fn has_wall(&self, direction: &HexDirection) -> bool {
        *self.walls.get(direction).unwrap_or(&false)
    }

    pub fn remove_wall(&mut self, direction: HexDirection) {
        self.walls.insert(direction, false);
    }
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            radius: 5,
            hex_size: Vec2::splat(32.),
        }
    }
}

pub fn setup_hex_grid(mut commands: Commands, grid_settings: Res<GridSettings>) {
    let GridSettings { radius, hex_size } = *grid_settings;
    let layout = HexLayout {
        orientation: HexOrientation::Pointy,
        origin: Vec2::ZERO,
        hex_size,
        ..default()
    };

    let hexes = Hex::ZERO.range(radius);

    for hex in hexes {
        let world_pos = layout.hex_to_world_pos(hex);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: GRAY.into(),
                    custom_size: Some(hex_size),
                    ..default()
                },
                transform: Transform::from_translation(world_pos.extend(0.)),
                ..default()
            },
            Tile::new(hex),
        ));
    }
}

pub fn generate_maze(mut tile_query: Query<&mut Tile>, grid_settings: Res<GridSettings>) {
    let radius = grid_settings.radius;
    let mut tiles = tile_query
        .iter_mut()
        .map(|tile| (tile.position, tile.clone()))
        .collect::<HashMap<Hex, Tile>>();

    let mut rng = thread_rng();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    let start_hex = Hex::ZERO;
    visited.insert(start_hex);
    stack.push(start_hex);

    while let Some(current_hex) = stack.pop() {
        let mut unvisited_neighbors = Vec::new();
        for direction in HexDirection::ALL {
            let neighbor_hex = current_hex + direction.to_hexx_direction();
            if neighbor_hex.distance_to(Hex::ZERO) > radius as i32 {
                continue;
            }
            if !visited.contains(&neighbor_hex) {
                unvisited_neighbors.push((neighbor_hex, direction));
            }
        }
        if !unvisited_neighbors.is_empty() {
            stack.push(current_hex);
            let &(neighbor_hex, direction) = unvisited_neighbors.iter().choose(&mut rng).unwrap();

            if let Some(current_tile) = tiles.get_mut(&current_hex) {
                current_tile.remove_wall(direction);
            }

            if let Some(neighbor_tile) = tiles.get_mut(&neighbor_hex) {
                neighbor_tile.remove_wall(direction.opposite());
            }

            visited.insert(neighbor_hex);
            stack.push(neighbor_hex);
        }
    }
}
