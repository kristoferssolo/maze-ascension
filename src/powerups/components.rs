use bevy::prelude::*;

/// Define the powerup types
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub enum Kind {
    #[default]
    WallJump,
    PathFinder,
}

/// Timer component for cooldowns
#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct Cooldown(Timer);

/// Timer component for duration
#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct Duration(Timer);

/// Active state component
#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct IsActive(bool);

/// Main powerup component that requires all other components
#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
#[require(Cooldown, Kind, IsActive)]
pub struct Powerup;

impl IsActive {
    pub const fn activate(&mut self) {
        self.0 = true;
    }

    pub const fn deactivate(&mut self) {
        self.0 = false;
    }
}

impl Default for Cooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(10., TimerMode::Once))
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self(Timer::from_seconds(3., TimerMode::Once))
    }
}

impl Default for IsActive {
    fn default() -> Self {
        Self(true)
    }
}
