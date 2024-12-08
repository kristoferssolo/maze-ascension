use bevy::{ecs::world::Command, prelude::*};
use plugin::MazePlugin;
mod assets;
mod components;
pub mod events;
pub mod plugin;
mod resources;
mod systems;

pub use resources::{MazeConfig, MazePluginLoaded};

pub fn spawn_maze(world: &mut World) {
    MazePlugin.apply(world);
}
