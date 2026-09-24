use bevy::prelude::*;
use hexx::Hex;

#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
#[require(CurrentPosition, MovementSpeed, MovementTarget)]
pub struct Player;

#[derive(Debug, Component, Deref, DerefMut, Default)]
pub struct CurrentPosition(pub Hex);

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct MovementSpeed(pub f32);

impl Default for MovementSpeed {
    fn default() -> Self {
        Self(100.)
    }
}

#[derive(Debug, Component, Deref, DerefMut, Default)]
pub struct MovementTarget(pub Option<Hex>);
