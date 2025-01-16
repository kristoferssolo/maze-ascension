mod score;
pub mod setup;

use bevy::prelude::*;
use score::{update_score, update_score_display};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_score, update_score_display)
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
}
