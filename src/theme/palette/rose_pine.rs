use super::rgb_u8;
use crate::theme::prelude::ColorScheme;
use bevy::prelude::*;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum RosePine {
    Base,
    Surface,
    Overlay,
    Muted,
    Subtle,
    Text,
    Love,
    Gold,
    Rose,
    Pine,
    Foam,
    Iris,
    HighlightLow,
    HighlightMed,
    HighlightHigh,
}

impl ColorScheme for RosePine {
    fn to_color(&self) -> Color {
        match self {
            Self::Base => rgb_u8(25, 23, 36),
            Self::Surface => rgb_u8(31, 29, 46),
            Self::Overlay => rgb_u8(38, 35, 58),
            Self::Muted => rgb_u8(110, 106, 134),
            Self::Subtle => rgb_u8(144, 140, 170),
            Self::Text => rgb_u8(224, 222, 244),
            Self::Love => rgb_u8(235, 111, 146),
            Self::Gold => rgb_u8(246, 193, 119),
            Self::Rose => rgb_u8(235, 188, 186),
            Self::Pine => rgb_u8(49, 116, 143),
            Self::Foam => rgb_u8(156, 207, 216),
            Self::Iris => rgb_u8(196, 167, 231),
            Self::HighlightLow => rgb_u8(33, 32, 46),
            Self::HighlightMed => rgb_u8(64, 61, 82),
            Self::HighlightHigh => rgb_u8(82, 79, 103),
        }
    }
}
