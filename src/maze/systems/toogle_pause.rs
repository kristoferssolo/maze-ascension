use bevy::prelude::*;

use crate::{maze::components::Wall, screens::Screen};

pub fn toggle_walls(mut query: Query<&mut Visibility, With<Wall>>, state: Res<State<Screen>>) {
    for mut visibility in query.iter_mut() {
        *visibility = match *state.get() {
            Screen::Gameplay => Visibility::Visible,
            Screen::Pause => Visibility::Hidden,
            _ => *visibility,
        }
    }
}
