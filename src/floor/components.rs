use bevy::prelude::*;

#[derive(Debug, Reflect, Component, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component)]
pub struct Floor(pub u8);

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
#[require(Floor)]
pub struct CurrentFloor;

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
#[require(Floor)]
pub struct FloorYTarget(pub f32);

impl Default for Floor {
    fn default() -> Self {
        Self(1)
    }
}

impl Floor {
    pub const fn increased(&self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub fn decreased(&self) -> Self {
        Self(self.0.saturating_sub(1).max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 1)]
    #[case(1, 2)]
    #[case(254, 255)]
    #[case(255, 255)]
    fn increase(#[case] input: u8, #[case] expected: u8) {
        let floor = Floor(input);
        assert_eq!(*floor.increased(), expected);
    }

    #[rstest]
    #[case(0, 1)] // clamps to 1
    #[case(1, 1)] // clamps to 1
    #[case(2, 1)]
    #[case(255, 254)]
    fn decrease(#[case] input: u8, #[case] expected: u8) {
        let floor = Floor(input);
        assert_eq!(*floor.decreased(), expected);
    }
}
