use bevy::prelude::*;

#[derive(Debug, Event)]
pub(crate) struct RecreateMazeEvent {
    pub(crate) floor: u8,
}
