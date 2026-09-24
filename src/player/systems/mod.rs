pub mod despawn;
mod input;
mod movement;
pub mod respawn;
mod sound_effect;
pub mod spawn;
mod toggle_pause;
mod vertical_transition;

use crate::{screens::Screen, AppSystems};
use bevy::prelude::*;
use input::player_input;
use movement::player_movement;
use sound_effect::play_movement_sound;
use toggle_pause::toggle_player;
use vertical_transition::handle_floor_transition;

use super::assets::PlayerAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_input.in_set(AppSystems::RecordInput),
            player_movement,
            handle_floor_transition.in_set(AppSystems::RecordInput),
            (play_movement_sound)
                .chain()
                .run_if(resource_exists::<PlayerAssets>)
                .in_set(AppSystems::Update),
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(Update, toggle_player.run_if(state_changed::<Screen>));
}
