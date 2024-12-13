use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct RecreateMazeEvent {
    pub floor: u8,
}
