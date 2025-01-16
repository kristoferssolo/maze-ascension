use bevy::prelude::*;

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct Score(pub usize);

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct StatsText;
