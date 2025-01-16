mod common;
mod floor;
mod floor_timer;
mod reset;
mod score;
pub mod spawn;
mod total_timer;

use bevy::prelude::*;
use floor::{update_floor_display, update_highest_floor_display};
use floor_timer::{update_floor_timer, update_floor_timer_display};
use reset::reset_timers;
use score::{update_score, update_score_display};
use total_timer::{update_total_timer, update_total_timer_display};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), reset_timers)
        .add_systems(
            Update,
            (
                (update_score, update_score_display).chain(),
                (update_floor_timer, update_floor_timer_display).chain(),
                (update_total_timer, update_total_timer_display).chain(),
                update_floor_display,
                update_highest_floor_display,
            )
                .run_if(in_state(Screen::Gameplay)),
        );
}
