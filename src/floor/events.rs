use bevy::prelude::*;

use super::components::Floor;

#[derive(Debug, Clone, Copy, Reflect, Event, Default, PartialEq, Eq)]
pub enum TransitionFloor {
    #[default]
    Ascend,
    Descend,
}

impl TransitionFloor {
    pub fn into_direction(&self) -> f32 {
        self.into()
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Ascend => Self::Descend,
            Self::Descend => Self::Ascend,
        }
    }

    pub fn next_floor_num(&self, floor: &Floor) -> u8 {
        match self {
            Self::Ascend => *floor.increased(),
            Self::Descend => *floor.decreased(),
        }
    }
}

impl From<TransitionFloor> for f32 {
    fn from(value: TransitionFloor) -> Self {
        f32::from(&value)
    }
}

impl From<&TransitionFloor> for f32 {
    fn from(value: &TransitionFloor) -> Self {
        match value {
            TransitionFloor::Ascend => -1.,
            TransitionFloor::Descend => 1.,
        }
    }
}
