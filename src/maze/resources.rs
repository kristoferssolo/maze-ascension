use std::num::TryFromIntError;

use bevy::prelude::*;
use hexx::{Hex, HexLayout, HexOrientation};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use thiserror::Error;

#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct MazePluginLoaded;

#[derive(Debug, Error)]
pub enum MazeConfigError {
    #[error("Failed to convert radius from u32 to i32: {0}")]
    RadiusConverions(#[from] TryFromIntError),
}

#[derive(Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct MazeConfig {
    pub radius: u32,
    pub height: f32,
    pub hex_size: f32,
    pub start_pos: Hex,
    pub end_pos: Hex,
    pub seed: u64,
    pub layout: HexLayout,
}

impl MazeConfig {
    fn new(
        radius: u32,
        height: f32,
        hex_size: f32,
        orientation: HexOrientation,
        seed: Option<u64>,
    ) -> Result<Self, MazeConfigError> {
        let seed = seed.unwrap_or_else(|| thread_rng().gen());
        let mut rng = StdRng::seed_from_u64(seed);

        let start_pos = generate_pos(radius, &mut rng)?;
        let end_pos = generate_pos(radius, &mut rng)?;

        debug!("Start pos: ({},{})", start_pos.x, start_pos.y);
        debug!("End pos: ({},{})", end_pos.x, end_pos.y);

        let layout = HexLayout {
            orientation,
            hex_size: Vec2::splat(hex_size),
            ..default()
        };

        Ok(Self {
            radius: radius as u32,
            height,
            hex_size,
            start_pos,
            end_pos,
            seed,
            layout,
        })
    }

    pub fn new_unchecked(
        radius: u32,
        height: f32,
        hex_size: f32,
        orientation: HexOrientation,
        seed: Option<u64>,
    ) -> Self {
        Self::new(radius, height, hex_size, orientation, seed)
            .expect("Failed to create MazeConfig with supposedly safe values")
    }

    pub fn wall_size(&self) -> f32 {
        self.hex_size / 6.
    }

    pub fn wall_offset(&self) -> f32 {
        self.hex_size - self.wall_size()
    }

    pub fn update(&mut self) {
        self.layout.hex_size = Vec2::splat(self.hex_size);
    }
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self::new_unchecked(7, 20., 6., HexOrientation::Flat, None)
    }
}

fn generate_pos<R: Rng>(radius: u32, rng: &mut R) -> Result<Hex, MazeConfigError> {
    let radius = i32::try_from(radius)?;
    Ok(Hex::new(
        rng.gen_range(-radius..radius),
        rng.gen_range(-radius..radius),
    ))
}
