//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use crate::{
    hint::assets::HintAssets,
    player::assets::PlayerAssets,
    screens::Screen,
    theme::{assets::InteractionAssets, prelude::*},
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        continue_to_title_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|parent| {
            parent.label("Loading...").insert(Node {
                justify_content: JustifyContent::Center,
                ..default()
            });
        });
}

fn continue_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

const fn all_assets_loaded(
    player_assets: Option<Res<PlayerAssets>>,
    interaction_assets: Option<Res<InteractionAssets>>,
    hints_assets: Option<Res<HintAssets>>,
) -> bool {
    player_assets.is_some() && interaction_assets.is_some() && hints_assets.is_some()
}
