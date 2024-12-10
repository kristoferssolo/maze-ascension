use bevy::prelude::*;
use hexx::Hex;

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
    pub current_hex: Hex,
    pub target_hex: Option<Hex>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 50.,
            current_hex: Hex::ZERO,
            target_hex: None,
        }
    }
}
