use crate::player::events::PlayerEvent;
use bevy::prelude::*;

pub(crate) fn setup(mut event_writer: EventWriter<PlayerEvent>) {
    event_writer.send(PlayerEvent::Spawn);
}
