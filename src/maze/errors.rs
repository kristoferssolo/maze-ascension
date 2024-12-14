use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MazeConfigError {
    #[error("Failed to convert radius from u32 to i32: {0}")]
    RadiusConverions(#[from] TryFromIntError),
    #[error("Invalid maze configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Error)]
pub enum MazeError {
    #[error("Floor {0} not found")]
    FloorNotFound(u8),
    #[error("Failed to generate maze with config: {radius}, seed: {seed}")]
    GenerationFailed { radius: u32, seed: u64 },
    #[error("Invalid tile entity: {0:?}")]
    TileNotFound(bevy::prelude::Entity),
    #[error("Failed to create maze assets")]
    AssetCreationFailed,
    #[error("Invalid maze configuration: {0}")]
    ConfigurationError(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type MazeResult<T> = Result<T, MazeError>;

impl MazeError {
    pub fn config_error(msg: impl Into<String>) -> Self {
        Self::ConfigurationError(msg.into())
    }

    pub fn generation_failed(radius: u32, seed: u64) -> Self {
        Self::GenerationFailed { radius, seed }
    }
}
