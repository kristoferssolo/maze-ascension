use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
#[reflect(Resource)]
pub struct TotalTimer(pub Timer);

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
#[reflect(Resource)]
pub struct FloorTimer(pub Timer);

impl Default for TotalTimer {
    fn default() -> Self {
        Self(init_timer())
    }
}

impl Default for FloorTimer {
    fn default() -> Self {
        Self(init_timer())
    }
}

fn init_timer() -> Timer {
    Timer::new(Duration::MAX, TimerMode::Once)
}
