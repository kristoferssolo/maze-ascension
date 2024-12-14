use crate::maze::{components::MazeConfig, events::MazeEvent};
use bevy::prelude::*;

pub(crate) fn setup(mut event_writer: EventWriter<MazeEvent>) {
    let config = MazeConfig::default();
    event_writer.send(MazeEvent::Create { floor: 1, config });
}
