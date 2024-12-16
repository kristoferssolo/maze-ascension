use bevy::prelude::*;

use super::components::MazeConfig;

#[derive(Debug, Event)]
pub enum MazeEvent {
    Create { floor: u8, config: MazeConfig },
    Recreate { floor: u8, config: MazeConfig },
    Remove { floor: u8 },
}
