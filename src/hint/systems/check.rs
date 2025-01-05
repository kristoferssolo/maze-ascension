use bevy::prelude::*;
use hexx::Hex;

use crate::{
    floor::components::CurrentFloor,
    hint::components::{Hint, IdleTimer},
    maze::components::MazeConfig,
    player::components::{CurrentPosition, MovementTarget, Player},
};

pub fn check_player_hints(
    mut idle_query: Query<&mut IdleTimer>,
    player_query: Query<(&CurrentPosition, &MovementTarget), With<Player>>,
    maze_query: Query<&MazeConfig, With<CurrentFloor>>,
    mut hint_query: Query<(&mut Visibility, &Hint)>,
    time: Res<Time>,
) {
    let Ok(mut idle_timer) = idle_query.get_single_mut() else {
        return;
    };

    let Ok(maze_config) = maze_query.get_single() else {
        return;
    };

    let Ok((player_pos, movement_target)) = player_query.get_single() else {
        return;
    };

    let is_moving = movement_target.is_some();

    if is_moving {
        // Reset timer and hide hints when player moves
        idle_timer.timer.reset();
        hide_all_hints(&mut hint_query, &mut idle_timer);
        return;
    }

    // Tick timer when player is idle
    idle_timer.timer.tick(time.delta());

    if idle_timer.timer.finished() {
        let on_special_tile = is_on_special_tile(player_pos, maze_config);

        if !idle_timer.movement_hint_visible {
            set_hint_visibility(&mut hint_query, Hint::Movement, true);
            idle_timer.movement_hint_visible = true;
        }
        if on_special_tile && !idle_timer.interaction_hint_visible {
            set_hint_visibility(&mut hint_query, Hint::Interaction, true);
            idle_timer.interaction_hint_visible = true;
        } else if !on_special_tile && idle_timer.interaction_hint_visible {
            set_hint_visibility(&mut hint_query, Hint::Interaction, false);
            idle_timer.interaction_hint_visible = false
        }
    }
}

fn hide_all_hints(hint_query: &mut Query<(&mut Visibility, &Hint)>, idle_timer: &mut IdleTimer) {
    for (mut visibility, _) in hint_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    idle_timer.hide_all();
}

fn set_hint_visibility(
    hint_query: &mut Query<(&mut Visibility, &Hint)>,
    hint: Hint,
    visible: bool,
) {
    for (mut visibility, hint_type) in hint_query.iter_mut() {
        if *hint_type == hint {
            *visibility = if visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        }
    }
}

fn is_on_special_tile(player_pos: &Hex, maze_config: &MazeConfig) -> bool {
    *player_pos == maze_config.start_pos || *player_pos == maze_config.end_pos
}
