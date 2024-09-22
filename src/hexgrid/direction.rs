use bevy::prelude::*;
use hexx::EdgeDirection;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Direction {
    Top,
    TopRight,
    BottomRight,
    Bottom,
    BottomLeft,
    TopLeft,
}

impl Direction {
    pub const ALL: [Direction; 6] = [
        Self::Top,
        Self::TopRight,
        Self::BottomRight,
        Self::Bottom,
        Self::BottomLeft,
        Self::TopLeft,
    ];

    pub fn opposite(&self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::TopRight => Self::BottomLeft,
            Self::BottomRight => Self::TopLeft,
            Self::Bottom => Self::Top,
            Self::BottomLeft => Self::TopRight,
            Self::TopLeft => Self::BottomRight,
        }
    }
}
