use bevy::prelude::*;

use crate::{
    constants::{
        BASE_FLOOR_SCORE, BASE_PERFECT_TIME, FLOOR_PROGRESSION_MULTIPLIER, MIN_TIME_MULTIPLIER,
        TIME_BONUS_MULTIPLIER, TIME_INCREASE_FACTOR,
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

    score.0 += calculate_score(
        hightes_floor.0.saturating_sub(1),
        floor_timer.elapsed_secs(),
    );
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
    let perfect_time = calculate_perfect_time(floor_number);

    // Floor progression using exponential scaling for better high-floor rewards
    let floor_multiplier = (1.0 + floor_number as f32).powf(FLOOR_PROGRESSION_MULTIPLIER);
    let base_score = BASE_FLOOR_SCORE as f32 * floor_multiplier;

    // Time bonus calculation
    // Perfect time or better gets maximum bonus
    // Longer times get diminishing returns but never below minimum
    let time_multiplier = if completion_time <= perfect_time {
        // Bonus for being faster than perfect time
        let speed_ratio = perfect_time / completion_time;
        speed_ratio * TIME_BONUS_MULTIPLIER
    } else {
        // Penalty for being slower than perfect time, with smooth degradation
        let overtime_ratio = completion_time / perfect_time;
        let time_factor = 1.0 / overtime_ratio;
        time_factor.max(MIN_TIME_MULTIPLIER) * TIME_BONUS_MULTIPLIER
    };

    dbg!(base_score * time_multiplier);

    (base_score * time_multiplier) as usize
}

/// Perfect time increases with floor number
fn calculate_perfect_time(floor_number: u8) -> f32 {
    BASE_PERFECT_TIME * (floor_number as f32 - 1.).mul_add(TIME_INCREASE_FACTOR, 1.)
}

#[cfg(test)]
mod tests {
    use claims::*;
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
    use rstest::*;

    use super::*;

    #[fixture]
    fn floors() -> Vec<u8> {
        (1..=100).collect()
    }

    #[fixture]
    fn times() -> Vec<f32> {
        vec![
            BASE_PERFECT_TIME * 0.5, // Much faster than perfect
            BASE_PERFECT_TIME * 0.8, // Faster than perfect
            BASE_PERFECT_TIME,       // Perfect time
            BASE_PERFECT_TIME * 1.5, // Slower than perfect
            BASE_PERFECT_TIME * 2.0, // Much slower
        ]
    }

    #[rstest]
    #[case(1, BASE_PERFECT_TIME)]
    #[case(2, BASE_PERFECT_TIME * (1.0 + TIME_INCREASE_FACTOR))]
    #[case(5, BASE_PERFECT_TIME * 4.0f32.mul_add(TIME_INCREASE_FACTOR, 1.))]
    fn specific_perfect_times(#[case] floor: u8, #[case] expected_time: f32) {
        let calculated_time = calculate_perfect_time(floor);
        assert_le!(
            (calculated_time - expected_time).abs(),
            0.001,
            "Perfect time calculation mismatch"
        );
    }

    #[rstest]
    fn score_progression(floors: Vec<u8>, times: Vec<f32>) {
        let floor_scores = floors
            .par_iter()
            .map(|floor| {
                let scores = times
                    .par_iter()
                    .map(|&time| (*floor, time, calculate_score(*floor, time)))
                    .collect::<Vec<_>>();
                (*floor, scores)
            })
            .collect::<Vec<_>>();

        for (floor, scores) in floor_scores {
            scores.windows(2).for_each(|window| {
                let (_, time1, score1) = window[0];
                let (_, time2, score2) = window[1];

                if time1 < time2 {
                    assert_gt!(
                        score1,
                        score2,
                        "Floor {}: Faster time ({}) should give higher score than slower time ({})",
                        floor,
                        time1,
                        time2
                    );
                }
            })
        }
    }

    #[rstest]
    fn perfect_time_progression(floors: Vec<u8>) {
        let perfect_scores = floors
            .par_iter()
            .map(|&floor| {
                let perfect_time = calculate_perfect_time(floor);
                (floor, calculate_score(floor, perfect_time))
            })
            .collect::<Vec<_>>();

        perfect_scores.windows(2).for_each(|window| {
            let (floor1, score1) = window[0];
            let (floor2, score2) = window[1];
            assert_gt!(
                score2,
                score1,
                "Floor {} perfect score ({}) should be higher than floor {} perfect score ({})",
                floor2,
                score2,
                floor1,
                score1
            );
        })
    }

    #[rstest]
    fn minimum_score_guarantee(floors: Vec<u8>) {
        let very_slow_time = BASE_PERFECT_TIME * 10.0;

        // Test minimum scores in parallel
        let min_scores = floors
            .par_iter()
            .map(|&floor| calculate_score(floor, very_slow_time))
            .collect::<Vec<_>>();

        // Verify minimum scores
        min_scores.windows(2).for_each(|window| {
            assert_gt!(
                window[1],
                window[0],
                "Higher floor should give better minimum score"
            );
        });

        // Verify all scores are above zero
        min_scores.iter().for_each(|&score| {
            assert_gt!(score, 0, "Score should never be zero");
        });
    }
}
