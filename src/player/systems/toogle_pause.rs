use bevy::prelude::*;

use crate::{player::components::Player, screens::Screen};

pub fn toggle_player(mut query: Query<&mut Visibility, With<Player>>, state: Res<State<Screen>>) {
    for mut visibility in query.iter_mut() {
        *visibility = match *state.get() {
            Screen::Gameplay => Visibility::Visible,
            Screen::Pause => Visibility::Hidden,
            _ => *visibility,
        }
    }
}
