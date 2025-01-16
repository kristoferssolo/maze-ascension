//! The screen state for the main gameplay.

use crate::{
    hint::spawn_hint_command, maze::spawn_level_command, player::spawn_player_command,
    screens::Screen, stats::spawn_stats_command,
};

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (
            spawn_level_command,
            spawn_player_command,
            spawn_hint_command,
            spawn_stats_command,
        )
            .chain(),
    );

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
