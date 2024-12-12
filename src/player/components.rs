use bevy::prelude::*;
use hexx::Hex;

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
#[require(CurrentPosition, MovementSpeed, MovementTarget)]
pub struct Player;

#[derive(Debug, Reflect, Component, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct CurrentPosition(pub Hex);

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct MovementSpeed(pub f32);

impl Default for MovementSpeed {
    fn default() -> Self {
        Self(50.)
    }
}

#[derive(Debug, Reflect, Component, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct MovementTarget(pub Option<Hex>);
