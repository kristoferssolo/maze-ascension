use bevy::prelude::*;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Floor(pub u8);

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct TargetFloor(pub u8);

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct CurrentFloor;

impl Default for Floor {
    fn default() -> Self {
        Self(1)
    }
}
