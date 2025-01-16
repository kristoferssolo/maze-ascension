use bevy::prelude::*;

use crate::{
    floor::resources::HighestFloor,
    stats::{components::FloorTimerDisplay, resources::FloorTimer},
};

use super::common::format_duration_adaptive;

pub fn update_floor_timer(
    mut floor_timer: ResMut<FloorTimer>,
    time: Res<Time>,
    hightes_floor: Res<HighestFloor>,
) {
    floor_timer.tick(time.delta());
    if hightes_floor.is_changed() {
        floor_timer.0.reset();
    }
}

pub fn update_floor_timer_display(
    mut text_query: Query<&mut Text, With<FloorTimerDisplay>>,
    floor_timer: Res<FloorTimer>,
) {
    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    text.0 = format!(
        "Floor Timer: {}",
        format_duration_adaptive(floor_timer.0.elapsed_secs())
    );
}
