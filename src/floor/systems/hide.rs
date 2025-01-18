use bevy::prelude::*;

use crate::floor::components::{CurrentFloor, Floor};

pub fn hide_upper_floors(
    mut query: Query<(&mut Visibility, &Floor)>,
    current_query: Query<&Floor, With<CurrentFloor>>,
) {
    let Ok(current_floor) = current_query.get_single() else {
        return;
    };
    for (mut visibility, floor) in query.iter_mut() {
        if floor > current_floor {
            *visibility = Visibility::Hidden
        } else {
            *visibility = Visibility::Visible
        }
    }
}
