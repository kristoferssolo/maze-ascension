use crate::floor::components::Floor;

use super::GlobalMazeConfig;
use bevy::prelude::*;
use hexlab::Maze;
use hexx::{Hex, HexLayout, HexOrientation};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
#[require(MazeConfig, Floor, Maze)]
pub struct HexMaze;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Tile;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Wall;

#[derive(Debug, Reflect, Component, Clone)]
#[reflect(Component)]
pub struct MazeConfig {
    pub radius: u16,
    pub start_pos: Hex,
    pub end_pos: Hex,
    pub seed: u64,
    pub layout: HexLayout,
}

impl MazeConfig {
    fn new(
        radius: u16,
        orientation: HexOrientation,
        seed: Option<u64>,
        global_conig: &GlobalMazeConfig,
    ) -> Self {
        let seed = seed.unwrap_or_else(|| thread_rng().gen());
        let mut rng = StdRng::seed_from_u64(seed);

        let start_pos = generate_pos(radius, &mut rng);
        let end_pos = generate_pos(radius, &mut rng);

        info!(
            "Start pos: (q={}, r={}). End pos: (q={}, r={})",
            start_pos.x, start_pos.y, end_pos.x, end_pos.y
        );

        let layout = HexLayout {
            orientation,
            hex_size: Vec2::splat(global_conig.hex_size),
            ..default()
        };

        Self {
            radius,
            start_pos,
            end_pos,
            seed,
            layout,
        }
    }

    pub fn update(&mut self, global_conig: &GlobalMazeConfig) {
        self.layout.hex_size = Vec2::splat(global_conig.hex_size);
    }
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self::new(8, HexOrientation::Flat, None, &GlobalMazeConfig::default())
    }
}

fn generate_pos<R: Rng>(radius: u16, rng: &mut R) -> Hex {
    let radius = radius as i32;
    Hex::new(
        rng.gen_range(-radius..radius),
        rng.gen_range(-radius..radius),
    )
}
