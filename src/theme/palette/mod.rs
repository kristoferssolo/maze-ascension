pub mod rose_pine;

use bevy::prelude::*;

const MAX_COLOR_VALUE: f32 = 255.;

pub(super) const fn rgb_u8(red: u8, green: u8, blue: u8) -> Color {
    Color::srgb(scale(red), scale(green), scale(blue))
}

const fn scale(value: u8) -> f32 {
    value as f32 / MAX_COLOR_VALUE
}
