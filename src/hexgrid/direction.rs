use bevy::prelude::*;
use hexx::EdgeDirection;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HexDirection {
    Top,
    TopRight,
    BottomRight,
    Bottom,
    BottomLeft,
    TopLeft,
}

impl HexDirection {
    pub fn to_hexx_direction(self) -> EdgeDirection {
        self.into()
    }

    pub const ALL: [HexDirection; 6] = [
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

impl From<HexDirection> for EdgeDirection {
    fn from(value: HexDirection) -> Self {
        match value {
            HexDirection::Top => Self::FLAT_NORTH,
            HexDirection::TopRight => Self::FLAT_NORTH_EAST,
            HexDirection::BottomRight => Self::FLAT_SOUTH_EAST,
            HexDirection::Bottom => Self::FLAT_SOUTH,
            HexDirection::BottomLeft => Self::FLAT_SOUTH_WEST,
            HexDirection::TopLeft => Self::FLAT_NORTH_WEST,
        }
    }
}
