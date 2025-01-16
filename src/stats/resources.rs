use bevy::prelude::*;

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
#[reflect(Resource)]
pub struct GameTimer(pub Timer);

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
#[reflect(Resource)]
pub struct FloorTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.0, TimerMode::Once))
    }
}

impl Default for FloorTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.0, TimerMode::Once))
    }
}
