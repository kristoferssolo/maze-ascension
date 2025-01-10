//! Maze components and configuration.
//!
//! Module defines the core components and configuration structures used
//! for maze generation and rendering, including hexagonal maze layouts,
//! tiles, walls, and maze configuration.
use super::{coordinates::is_within_radius, GlobalMazeConfig};
use crate::floor::components::Floor;

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

/// Configuration for a single maze instance.
///
/// Contains all necessary parameters to generate and position a maze,
/// including its size, start/end positions, random seed, and layout.
#[derive(Debug, Reflect, Component, Clone)]
#[reflect(Component)]
pub struct MazeConfig {
    /// Radius of the hexagonal maze
    pub radius: u16,
    /// Starting position in hex coordinates
    pub start_pos: Hex,
    /// Ending position in hex coordinates
    pub end_pos: Hex,
    /// Random seed for maze generation
    pub seed: u64,
    /// Layout configuration for hex-to-world space conversion
    pub layout: HexLayout,
}

impl MazeConfig {
    /// Creates a new maze configuration with the specified parameters.
    fn new(
        radius: u16,
        orientation: HexOrientation,
        seed: Option<u64>,
        global_config: &GlobalMazeConfig,
        start_pos: Option<Hex>,
    ) -> Self {
        let (seed, mut rng) = setup_rng(seed);

        let start_pos = start_pos.unwrap_or_else(|| generate_pos(radius, &mut rng));

        // Generate end position ensuring start and end are different
        let end_pos = generate_end_pos(radius, start_pos, &mut rng);

        let layout = HexLayout {
            orientation,
            hex_size: Vec2::splat(global_config.hex_size),
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

    pub fn from_self(config: &Self) -> Self {
        let start_pos = config.end_pos;
        let (seed, mut rng) = setup_rng(None);

        let end_pos = generate_end_pos(config.radius, start_pos, &mut rng);

        Self {
            radius: config.radius + 1,
            start_pos,
            end_pos,
            seed,
            layout: config.layout.clone(),
        }
    }

    /// Updates the maze configuration with new global settings.
    pub fn update(&mut self, global_conig: &GlobalMazeConfig) {
        self.layout.hex_size = Vec2::splat(global_conig.hex_size);
    }
}

// TO
// 3928551514041614914
// (4, 0)

// FROM
// 7365371276044996661
// ()

impl Default for MazeConfig {
    fn default() -> Self {
        Self::new(
            4,
            HexOrientation::Flat,
            None,
            &GlobalMazeConfig::default(),
            None,
        )
    }
}

fn setup_rng(seed: Option<u64>) -> (u64, StdRng) {
    let seed = seed.unwrap_or_else(|| thread_rng().gen());
    let rng = StdRng::seed_from_u64(seed);
    (seed, rng)
}

fn generate_end_pos<R: Rng>(radius: u16, start_pos: Hex, rng: &mut R) -> Hex {
    let mut end_pos;
    loop {
        end_pos = generate_pos(radius, rng);
        if start_pos != end_pos {
            return end_pos;
        }
    }
}

/// Generates a random position within a hexagonal radius.
///
/// # Returns
/// A valid Hex coordinate within the specified radius
fn generate_pos<R: Rng>(radius: u16, rng: &mut R) -> Hex {
    let radius = radius as i32;

    loop {
        // Generate coordinates using cube coordinate bounds
        let q = rng.gen_range(-radius..=radius);
        let r = rng.gen_range((-radius).max(-q - radius)..=radius.min(-q + radius));

        if let Ok(is_valid) = is_within_radius(radius, &(q, r)) {
            if is_valid {
                return Hex::new(q, r);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::*;
    use rstest::*;

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(5)]
    #[case(8)]
    fn maze_config_new(#[case] radius: u16) {
        let orientation = HexOrientation::Flat;
        let seed = Some(12345);
        let global_config = GlobalMazeConfig::default();

        let config = MazeConfig::new(radius, orientation, seed, &global_config, None);

        assert_eq!(config.radius, radius);
        assert_eq!(config.seed, 12345);
        assert_eq!(config.layout.orientation, orientation);

        assert_ok!(is_within_radius(radius, &config.start_pos),);
        assert_ok!(is_within_radius(radius, &config.end_pos));
        assert_ne!(config.start_pos, config.end_pos);
    }

    #[rstest]
    #[case(100)]
    fn maze_config_default(#[case] iterations: u32) {
        for _ in 0..iterations {
            let config = MazeConfig::default();
            let radius = config.radius;

            assert_ok!(is_within_radius(radius, &config.start_pos));
            assert_ok!(is_within_radius(radius, &config.end_pos));
            assert_ne!(config.start_pos, config.end_pos);
        }
    }

    #[test]
    fn maze_config_default_with_seeds() {
        let test_seeds = [
            None,
            Some(0),
            Some(1),
            Some(12345),
            Some(u64::MAX),
            Some(thread_rng().gen()),
        ];

        for seed in test_seeds {
            let config = MazeConfig::new(
                8,
                HexOrientation::Flat,
                seed,
                &GlobalMazeConfig::default(),
                None,
            );

            assert_eq!(config.radius, 8);
            assert_eq!(config.layout.orientation, HexOrientation::Flat);
            assert_ok!(is_within_radius(8, &config.start_pos));
            assert_ok!(is_within_radius(8, &config.end_pos));
            assert_ne!(config.start_pos, config.end_pos);
        }
    }

    #[rstest]
    #[case(1.0)]
    #[case(2.0)]
    #[case(5.0)]
    fn maze_config_update(#[case] new_size: f32) {
        let mut config = MazeConfig::default();
        let global_config = GlobalMazeConfig {
            hex_size: new_size,
            ..default()
        };

        config.update(&global_config);

        assert_eq!(config.layout.hex_size.x, new_size);
        assert_eq!(config.layout.hex_size.y, new_size);
    }

    #[rstest]
    #[case(5, 1)]
    #[case(5, 12345)]
    #[case(8, 67890)]
    fn generate_pos_with_seed(#[case] radius: u16, #[case] seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);

        for _ in 0..10 {
            let pos = generate_pos(radius, &mut rng);
            assert_ok!(is_within_radius(radius, &pos),);
        }
    }

    #[test]
    fn different_seeds_different_positions() {
        let config1 = MazeConfig::new(
            8,
            HexOrientation::Flat,
            Some(1),
            &GlobalMazeConfig::default(),
            None,
        );
        let config2 = MazeConfig::new(
            8,
            HexOrientation::Flat,
            Some(2),
            &GlobalMazeConfig::default(),
            None,
        );

        assert_ne!(config1.start_pos, config2.start_pos);
        assert_ne!(config1.end_pos, config2.end_pos);
    }

    #[test]
    fn same_seed_same_positions() {
        let seed = Some(12345);
        let config1 = MazeConfig::new(
            8,
            HexOrientation::Flat,
            seed,
            &GlobalMazeConfig::default(),
            None,
        );
        let config2 = MazeConfig::new(
            8,
            HexOrientation::Flat,
            seed,
            &GlobalMazeConfig::default(),
            None,
        );

        assert_eq!(config1.start_pos, config2.start_pos);
        assert_eq!(config1.end_pos, config2.end_pos);
    }

    #[test]
    fn orientation_pointy() {
        let config = MazeConfig::new(
            8,
            HexOrientation::Pointy,
            None,
            &GlobalMazeConfig::default(),
            None,
        );
        assert_eq!(config.layout.orientation, HexOrientation::Pointy);
    }

    #[test]
    fn hex_size_zero() {
        let config = MazeConfig::new(
            8,
            HexOrientation::Flat,
            None,
            &GlobalMazeConfig {
                hex_size: 0.0,
                ..default()
            },
            None,
        );
        assert_eq!(config.layout.hex_size.x, 0.0);
        assert_eq!(config.layout.hex_size.y, 0.0);
    }

    #[test]
    fn basic_generation() {
        let mut rng = thread_rng();
        let radius = 2;
        let hex = generate_pos(radius, &mut rng);

        // Test that generated position is within radius
        assert_ok!(is_within_radius(radius as i32, &(hex.x, hex.y)));
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(6)]
    fn multiple_radii(#[case] radius: u16) {
        let mut rng = thread_rng();

        // Generate multiple points for each radius
        for _ in 0..100 {
            let hex = generate_pos(radius, &mut rng);
            assert_ok!(is_within_radius(radius, &hex));
        }
    }

    #[test]
    fn zero_radius() {
        let mut rng = thread_rng();
        let hex = generate_pos(0, &mut rng);

        // With radius 0, only (0,0) should be possible
        assert_eq!(hex.x, 0);
        assert_eq!(hex.y, 0);
    }

    #[test]
    fn large_radius() {
        let mut rng = thread_rng();
        let radius = 100;
        let iterations = 100;

        for _ in 0..iterations {
            let hex = generate_pos(radius, &mut rng);
            assert_ok!(is_within_radius(radius, &hex));
        }
    }
}
