use bevy::prelude::*;

use crate::stats::{components::TotalTimerDisplay, resources::TotalTimer};

use super::common::format_duration_adaptive;

pub fn update_total_timer(mut total_timer: ResMut<TotalTimer>, time: Res<Time>) {
    total_timer.tick(time.delta());
}

pub fn update_total_timer_display(
    mut text_query: Query<&mut Text, With<TotalTimerDisplay>>,
    total_timer: Res<TotalTimer>,
) {
    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    text.0 = format!(
        "Total Timer: {}",
        format_duration_adaptive(total_timer.0.elapsed_secs())
    );
}
