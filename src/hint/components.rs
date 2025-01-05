use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Reflect, Component, PartialEq, Eq)]
#[reflect(Component)]
pub enum Hint {
    Movement,
    Interaction,
}

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct IdleTimer {
    pub timer: Timer,
    pub movement_hint_visible: bool,
    pub interaction_hint_visible: bool,
}

impl IdleTimer {
    pub fn hide_all(&mut self) {
        self.movement_hint_visible = false;
        self.interaction_hint_visible = false;
    }
}

impl Default for IdleTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
            movement_hint_visible: false,
            interaction_hint_visible: false,
        }
    }
}
