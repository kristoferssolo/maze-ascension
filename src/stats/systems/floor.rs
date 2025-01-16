use bevy::prelude::*;

use crate::{
    floor::{
        components::{CurrentFloor, Floor},
        resources::HighestFloor,
    },
    stats::components::{FloorDisplay, HighestFloorDisplay},
};

pub fn update_floor_display(
    floor_query: Query<&Floor, With<CurrentFloor>>,
    mut text_query: Query<&mut Text, With<FloorDisplay>>,
) {
    let Ok(floor) = floor_query.get_single() else {
        return;
    };

    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    text.0 = format!("Floor: {}", floor.0);
}

pub fn update_highest_floor_display(
    hightes_floor: Res<HighestFloor>,
    mut text_query: Query<&mut Text, With<HighestFloorDisplay>>,
) {
    if !hightes_floor.is_changed() {
        return;
    }

    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    text.0 = format!("Highest Floor: {}", hightes_floor.0);
}
