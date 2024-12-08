use std::num::TryFromIntError;

use bevy::prelude::*;
use hexx::{Hex, HexLayout, HexOrientation};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use thiserror::Error;

#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct MazePluginLoaded;

pub(crate) const WALL_SIZE: f32 = 1.0;
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
    pub size: f32,
    pub start_pos: Hex,
    pub end_pos: Hex,
    pub seed: u64,
}

impl MazeConfig {
    fn new(
        radius: u32,
        height: f32,
        size: f32,
        seed: Option<u64>,
    ) -> Result<Self, MazeConfigError> {
        let seed = seed.unwrap_or_else(|| thread_rng().gen());
        let mut rng = StdRng::seed_from_u64(seed);

        let start_pos = generate_pos(radius, &mut rng)?;
        let end_pos = generate_pos(radius, &mut rng)?;

        debug!("Start pos: ({},{})", start_pos.x, start_pos.y);
        debug!("End pos: ({},{})", end_pos.x, end_pos.y);

        Ok(Self {
            radius: radius as u32,
            height,
            size,
            start_pos,
            end_pos,
            seed,
        })
    }

    pub fn new_unchecked(radius: u32, height: f32, hex_size: f32, seed: Option<u64>) -> Self {
        Self::new(radius, height, hex_size, seed)
            .expect("Failed to create MazeConfig with supposedly safe values")
    }
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self::new_unchecked(7, 20., 6., None)
    }
}

#[derive(Debug, Reflect, Resource, Deref, DerefMut, Clone)]
#[reflect(Resource)]
pub struct Layout(pub HexLayout);

impl FromWorld for Layout {
    fn from_world(world: &mut World) -> Self {
        let config = world.resource::<MazeConfig>();
        Self(HexLayout {
            orientation: HexOrientation::Flat,
            hex_size: Vec2::splat(config.size),
            ..default()
        })
    }
}

fn generate_pos<R: Rng>(radius: u32, rng: &mut R) -> Result<Hex, MazeConfigError> {
    let radius = i32::try_from(radius)?;
    Ok(Hex::new(
        rng.gen_range(-radius..radius),
        rng.gen_range(-radius..radius),
    ))
}
