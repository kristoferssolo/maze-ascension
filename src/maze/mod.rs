use bevy::{ecs::world::Command, prelude::*};
use plugin::MazePlugin;
mod assets;
mod components;
pub mod events;
pub mod plugin;
mod resources;
mod systems;

pub use resources::MazeConfig;

pub fn spawn_grid(world: &mut World) {
    MazePlugin.apply(world);
}
