use bevy::prelude::*;

use crate::{
    screens::Screen,
    stats::{
        components::{
            FloorDisplay, FloorTimerDisplay, HighestFloorDisplay, ScoreDisplay, TotalTimerDisplay,
        },
        container::StatsContainer,
    },
    theme::widgets::Widgets,
};

pub fn spawn_stats(mut commands: Commands) {
    commands
        .ui_stats()
        .insert(StateScoped(Screen::Gameplay))
        .with_children(|parent| {
            parent.stats("Floor: 1", FloorDisplay);
            parent.stats("Highest Floor: 1", HighestFloorDisplay);
            parent.stats("Score: 0", ScoreDisplay);
            parent.stats("Floor Timer", FloorTimerDisplay);
            parent.stats("Total Timer", TotalTimerDisplay);
        });
}
