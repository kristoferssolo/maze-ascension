//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod pause;
mod splash;
mod title;

use bevy::prelude::*;
pub use gameplay::{GameplayElement, GameplayInitialized};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        pause::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[cfg_attr(not(feature = "dev"), default)]
    Splash,
    #[cfg_attr(feature = "dev", default)]
    Loading,
    Title,
    Gameplay,
    Pause,
}
