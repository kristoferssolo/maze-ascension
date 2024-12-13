use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct MazePluginLoaded;

#[derive(Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct GlobalMazeConfig {
    pub hex_size: f32,
    pub wall_thickness: f32,
    pub height: f32,
}

impl GlobalMazeConfig {
    pub fn wall_size(&self) -> f32 {
        self.hex_size / 6.
    }

    pub fn wall_offset(&self) -> f32 {
        self.hex_size - self.wall_size()
    }
}

impl Default for GlobalMazeConfig {
    fn default() -> Self {
        Self {
            hex_size: 6.,
            wall_thickness: 1.,
            height: 20.,
        }
    }
}
