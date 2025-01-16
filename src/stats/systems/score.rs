use bevy::prelude::*;

use crate::{
    constants::{FLOOR_SCORE_MULTIPLIER, TIME_SCORE_MULTIPLIER},
    floor::components::{CurrentFloor, Floor},
    stats::{components::Score, resources::GameTimer},
};

pub fn update_score(
    mut score_query: Query<&mut Score>,
    mut game_timer: ResMut<GameTimer>,
    floor_query: Query<&Floor, With<CurrentFloor>>,
    time: Res<Time>,
) {
    game_timer.tick(time.delta());

    let Ok(mut score) = score_query.get_single_mut() else {
        return;
    };

    let Ok(current_floor) = floor_query.get_single() else {
        return;
    };

    let time_score = game_timer.elapsed_secs() * TIME_SCORE_MULTIPLIER;
    let floor_score = current_floor.0 as f32 * FLOOR_SCORE_MULTIPLIER;
    score.0 = (time_score + floor_score) as usize;
}

pub fn update_score_display(score_query: Query<&Score>, mut text_query: Query<&mut Text>) {
    let Ok(score) = score_query.get_single() else {
        return;
    };
    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    text.0 = format!("Score: {}", score.0);
}
