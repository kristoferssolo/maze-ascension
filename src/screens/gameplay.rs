//! The screen state for the main gameplay.

use crate::{
    hint::spawn_hint_command, maze::spawn_level_command, player::spawn_player_command,
    screens::Screen, stats::spawn_stats_command,
};

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameplayInitialized>();
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (
            spawn_level_command,
            spawn_player_command,
            spawn_hint_command,
            spawn_stats_command,
        )
            .chain()
            .run_if(not(resource_exists::<GameplayInitialized>)),
    );
    app.add_systems(OnEnter(Screen::Gameplay), |mut commands: Commands| {
        commands.insert_resource(GameplayInitialized(true));
    });

    app.add_systems(OnEnter(Screen::Title), reset_gameplay_state);

    app.add_systems(
        Update,
        pause_game.run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn pause_game(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Pause);
}

fn reset_gameplay_state(mut commands: Commands) {
    commands.remove_resource::<GameplayInitialized>();
}

#[derive(Debug, Default, Reflect, Resource, DerefMut, Deref)]
#[reflect(Resource)]
pub struct GameplayInitialized(bool);
