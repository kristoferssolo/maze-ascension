use bevy::prelude::*;

use crate::{
    stats::{components::Score, stats::StatsContainer},
    theme::widgets::Widgets,
};

pub fn setup(mut commands: Commands) {
    commands.spawn((Name::new("Score"), Score(0)));

    commands.ui_stats().with_children(|parent| {
        parent.stats("Floor", "0");
        parent.stats("Score", "0");
        parent.stats("Floor timer", "00:00");
        parent.stats("Game timer", "00:00");
    });
}
