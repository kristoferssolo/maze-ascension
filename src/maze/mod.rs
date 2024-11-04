use bevy::{ecs::world::Command, prelude::*};
use plugin::MazePlugin;
pub mod grid;
pub mod plugin;
pub mod prism;
pub mod resource;
pub mod tile;

pub fn spawn_grid(world: &mut World) {
    MazePlugin.apply(world);
}
