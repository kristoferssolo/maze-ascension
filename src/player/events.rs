use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct SpawnPlayer;

#[derive(Debug, Event)]
pub struct RespawnPlayer;

#[derive(Debug, Event)]
pub struct DespawnPlayer;
