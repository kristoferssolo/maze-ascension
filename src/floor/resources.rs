use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Resource, PartialEq, Eq)]
#[reflect(Resource)]
pub struct HighestFloor(pub u8);
