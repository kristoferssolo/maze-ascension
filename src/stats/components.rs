use bevy::prelude::*;

#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct FloorDisplay;

#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct HighestFloorDisplay;

#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct ScoreDisplay;

#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct FloorTimerDisplay;

#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct TotalTimerDisplay;
