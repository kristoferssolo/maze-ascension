use std::fmt::Display;

use bevy::prelude::*;
use hexx::{Hex, HexLayout};

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

    pub fn _visit(&mut self) {
        self.visited = true;
    }

    pub fn to_vec2(&self, layout: &HexLayout) -> Vec2 {
        layout.hex_to_world_pos(self.hex)
    }

    pub fn to_vec3(&self, layout: &HexLayout) -> Vec3 {
        let pos = self.to_vec2(layout);
        Vec3::new(pos.x, 0., pos.y)
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
