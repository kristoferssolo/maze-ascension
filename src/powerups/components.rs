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
#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Cooldown(Timer);

/// Active state component
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Active(bool);

/// Main powerup component that requires all other components
#[derive(Debug, Reflect, Component)]
#[require(Cooldown, Kind, Active)]
pub struct Powerup;

impl Default for Cooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(10., TimerMode::Once))
    }
}
