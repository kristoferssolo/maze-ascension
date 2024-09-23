use std::fmt::Display;

use bevy::prelude::*;
use hexx::Hex;

#[derive(Debug, Reflect, Component, Default, PartialEq, Eq, Hash, Clone)]
#[reflect(Component)]
pub struct Tile {
    pub hex: Hex,
    pub visited: bool,
}

#[derive(Debug, Reflect, Component, Deref, DerefMut, Clone)]
#[reflect(Component)]
pub struct Walls(pub [bool; 6]);

#[derive(Debug, Reflect, Bundle, Default)]
pub struct TileBundle {
    pub hex: Tile,
    pub walls: Walls,
}

impl Tile {
    pub fn new(q: i32, r: i32) -> Self {
        Self {
            hex: Hex::new(q, r),
            visited: false,
        }
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.hex.x, self.hex.y)
    }
}

impl Default for Walls {
    fn default() -> Self {
        Self([true; 6])
    }
}
