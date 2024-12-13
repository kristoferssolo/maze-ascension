use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MazeConfigError {
    #[error("Failed to convert radius from u32 to i32: {0}")]
    RadiusConverions(#[from] TryFromIntError),
    #[error("Invalid maze configuration: {0}")]
    InvalidConfig(String),
}
