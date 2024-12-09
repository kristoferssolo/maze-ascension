use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
}
