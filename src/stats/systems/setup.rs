use bevy::prelude::*;

use crate::{
    stats::{
        components::{
            FloorDisplay, FloorTimerDisplay, HighestFloorDisplay, Score, ScoreDisplay,
            TotalTimerDisplay,
        },
        container::StatsContainer,
    },
    theme::widgets::Widgets,
};

pub fn setup(mut commands: Commands) {
    commands.spawn((Name::new("Score"), Score(0)));
    commands.ui_stats().with_children(|parent| {
        parent.stats("Floor: 1", FloorDisplay);
        parent.stats("Highest Floor: 1", HighestFloorDisplay);
        parent.stats("Score: 0", ScoreDisplay);
        parent.stats("Floor Timer", FloorTimerDisplay);
        parent.stats("Total Timer", TotalTimerDisplay);
    });
}
