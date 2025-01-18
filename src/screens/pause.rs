use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::theme::{
    events::OnPress,
    palette::rose_pine::RosePineDawn,
    prelude::ColorScheme,
    widgets::{Containers, Widgets},
};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Pause), spawn_pause_overlay);
    app.add_systems(
        Update,
        return_to_game.run_if(in_state(Screen::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands
        .ui_root()
        .insert((
            StateScoped(Screen::Pause),
            BackgroundColor(RosePineDawn::Muted.to_color().with_alpha(0.5)),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    bottom: Val::Px(100.),
                    ..default()
                })
                .with_children(|parent| {
                    parent.header("Paused");
                });

            parent.button("Continue").observe(return_to_game_trigger);
            parent
                .button("Exit")
                .observe(return_to_title_screen_trigger);
        });
}

fn return_to_game_trigger(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn return_to_title_screen_trigger(
    _trigger: Trigger<OnPress>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Title);
}

fn return_to_game(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}
