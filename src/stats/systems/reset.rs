use bevy::prelude::*;

use crate::stats::resources::{FloorTimer, Score, TotalTimer};

pub fn reset_timers(
    mut floor_timer: ResMut<FloorTimer>,
    mut total_timer: ResMut<TotalTimer>,
    mut score: ResMut<Score>,
) {
    floor_timer.reset();
    total_timer.reset();
    score.0 = 0;
}
