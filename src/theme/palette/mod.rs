pub mod rose_pine;

use bevy::prelude::*;

pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.186, 0.328, 0.573);
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.286, 0.478, 0.773);

pub const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);
pub const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);
pub const HEADER_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);

pub const NODE_BACKGROUND: Color = Color::srgb(0.286, 0.478, 0.773);

const MAX_COLOR_VALUE: f32 = 255.;

pub(super) const fn rgb_u8(red: u8, green: u8, blue: u8) -> Color {
    Color::srgb(scale(red), scale(green), scale(blue))
}

const fn scale(value: u8) -> f32 {
    value as f32 / MAX_COLOR_VALUE
}
