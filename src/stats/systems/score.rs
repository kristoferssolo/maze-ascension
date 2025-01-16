use bevy::prelude::*;

use crate::{
    constants::{
        BASE_FLOOR_SCORE, FLOOR_DIFFICULTY_MULTIPLIER, MIN_TIME_MULTIPLIER, TIME_REFERENCE_SECONDS,
    },
    floor::resources::HighestFloor,
    stats::{
        components::ScoreDisplay,
        resources::{FloorTimer, Score},
    },
};

pub fn update_score(
    mut score: ResMut<Score>,
    hightes_floor: Res<HighestFloor>,
    floor_timer: Res<FloorTimer>,
) {
    if !hightes_floor.is_changed() || hightes_floor.is_added() {
        return;
    }

    score.0 = calculate_score(hightes_floor.0, floor_timer.elapsed_secs());
}

pub fn update_score_display(
    mut text_query: Query<&mut Text, With<ScoreDisplay>>,
    score: Res<Score>,
) {
    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    text.0 = format!("Score: {}", score.0);
}

fn calculate_score(floor_number: u8, completion_time: f32) -> usize {
    // Calculate base floor score with exponential scaling
    let floor_multiplier = (floor_number as f32).powf(FLOOR_DIFFICULTY_MULTIPLIER);
    let base_score = BASE_FLOOR_SCORE as f32 * floor_multiplier;

    // Calculate time multiplier (decreases as time increases)
    let time_factor = 1. / (1. + (completion_time / TIME_REFERENCE_SECONDS));
    let time_multiplier = time_factor.max(MIN_TIME_MULTIPLIER);

    (base_score * time_multiplier) as usize
}
