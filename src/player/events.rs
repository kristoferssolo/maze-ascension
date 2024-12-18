use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct SpawnPlayer;

#[derive(Debug, Event)]
pub struct RespawnPlayer;

#[derive(Debug, Event)]
pub struct DespawnPlayer;

#[derive(Debug, Event)]
pub struct AscendPlayer {
    pub floor: u8,
}

#[derive(Debug, Event)]
pub struct DescendPlayer {
    pub floor: u8,
}
