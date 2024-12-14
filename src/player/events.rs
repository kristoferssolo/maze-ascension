use bevy::prelude::*;

#[derive(Debug, Event)]
pub enum PlayerEvent {
    Spawn,
    Respawn,
    Despawn,
}
