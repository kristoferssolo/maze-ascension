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
        return_to_game
            .run_if(in_state(Screen::Pause).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands
        .ui_root()
        .insert((
            DespawnOnExit(Screen::Pause),
            // The HUD is a separate UI root, so local ZIndex cannot cover it.
            GlobalZIndex(1),
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

fn return_to_game_trigger(_trigger: On<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn return_to_title_screen_trigger(
    _trigger: On<OnPress>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Title);
}

fn return_to_game(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_ok;

    #[test]
    fn pause_overlay_stacks_above_gameplay_ui() {
        let mut app = App::new();
        app.add_systems(Update, spawn_pause_overlay);

        app.update();

        let world = app.world_mut();
        let mut overlays = world.query_filtered::<&GlobalZIndex, With<BackgroundColor>>();
        assert!(assert_ok!(overlays.single(world)).0 > 0);
    }
}
