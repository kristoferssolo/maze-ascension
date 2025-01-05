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
        start_pos: Option<Hex>,
    ) -> Self {
        let seed = seed.unwrap_or_else(|| thread_rng().gen());
        let mut rng = StdRng::seed_from_u64(seed);

        let start_pos = start_pos.unwrap_or_else(|| generate_pos(radius, &mut rng));

        // Generate end position ensuring start and end are different
        let mut end_pos;
        loop {
            end_pos = generate_pos(radius, &mut rng);
            if start_pos != end_pos {
                break;
            }
        }

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
        Self::new(
            4,
            HexOrientation::Flat,
            None,
            &GlobalMazeConfig::default(),
            None,
        )
    }
}

fn generate_pos<R: Rng>(radius: u16, rng: &mut R) -> Hex {
    let radius = radius as i32;
    loop {
        let q = rng.gen_range(-radius..=radius);
        let r = rng.gen_range(-radius..=radius);
        let s = -q - r; // Calculate third coordinate (axial coordinates: q + r + s = 0)

        // Check if the position is within the hexagonal radius
        // Using the formula: max(abs(q), abs(r), abs(s)) <= radius
        if q.abs().max(r.abs()).max(s.abs()) <= radius {
            return Hex::new(q, r);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn is_within_radius(hex: Hex, radius: u16) -> bool {
        let q = hex.x;
        let r = hex.y;
        let s = -q - r;
        q.abs().max(r.abs()).max(s.abs()) <= radius as i32
    }

    #[fixture]
    fn test_radius() -> Vec<u16> {
        vec![1, 2, 5, 8]
    }

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

        assert!(
            is_within_radius(config.start_pos, radius),
            "Start pos {:?} outside radius {}",
            config.start_pos,
            radius
        );
        assert!(
            is_within_radius(config.end_pos, radius),
            "End pos {:?} outside radius {}",
            config.end_pos,
            radius
        );
        assert_ne!(config.start_pos, config.end_pos);
    }

    #[rstest]
    #[case(100)]
    fn maze_config_default(#[case] iterations: u32) {
        for _ in 0..iterations {
            let config = MazeConfig::default();

            assert_eq!(config.radius, 8);
            assert_eq!(config.layout.orientation, HexOrientation::Flat);
            assert!(is_within_radius(config.start_pos, 8));
            assert!(is_within_radius(config.end_pos, 8));
            assert_ne!(config.start_pos, config.end_pos);
        }
    }

    #[rstest]
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
            assert!(is_within_radius(config.start_pos, 8));
            assert!(is_within_radius(config.end_pos, 8));
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
            assert!(
                is_within_radius(pos, radius),
                "Position {:?} outside radius {}",
                pos,
                radius
            );
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
}
