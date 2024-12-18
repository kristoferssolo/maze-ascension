use bevy::prelude::*;

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct Floor(pub u8);

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
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

impl Floor {
    pub fn increase(&self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub fn decrease(&self) -> Self {
        Self(self.0.saturating_sub(1).max(1))
    }
}
