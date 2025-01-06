mod check;
pub mod spawn;

use bevy::prelude::*;
use check::check_player_hints;

use super::assets::HintAssets;
use crate::{asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<HintAssets>();
    app.add_systems(
        Update,
        check_player_hints.run_if(in_state(Screen::Gameplay)),
    );
}
