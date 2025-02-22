//! The title screen that appears when the game starts.

use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

fn spawn_title_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    bottom: Val::Px(70.),
                    ..default()
                })
                .with_children(|parent| {
                    parent.header("Maze Ascension");
                });
            parent.button("Play").observe(enter_gameplay_screen);

            #[cfg(not(target_family = "wasm"))]
            parent.button("Quit").observe(exit_app);
        });
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
