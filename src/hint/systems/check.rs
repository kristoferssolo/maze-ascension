use bevy::prelude::*;
use hexx::Hex;

use crate::{
    floor::components::{CurrentFloor, Floor, FloorYTarget},
    hint::components::{Hint, IdleTimer},
    maze::components::MazeConfig,
    player::components::{CurrentPosition, MovementTarget, Player},
};

pub fn check_player_hints(
    mut idle_query: Query<&mut IdleTimer>,
    player_query: Query<(&CurrentPosition, &MovementTarget), With<Player>>,
    tranitioning: Query<Entity, With<FloorYTarget>>,
    maze_query: Query<(&MazeConfig, &Floor), With<CurrentFloor>>,
    mut hint_query: Query<(&mut Visibility, &Hint)>,
    time: Res<Time>,
) {
    let Ok(mut idle_timer) = idle_query.single_mut() else {
        return;
    };

    let Ok((maze_config, floor)) = maze_query.single() else {
        return;
    };

    let Ok((player_pos, movement_target)) = player_query.single() else {
        return;
    };

    let is_moving = movement_target.is_some() || tranitioning.iter().next().is_some();

    if is_moving {
        // Reset timer and hide hints when player moves
        idle_timer.timer.reset();
        hide_all_hints(hint_query, &mut idle_timer);
        return;
    }

    // Tick timer when player is idle
    idle_timer.timer.tick(time.delta());

    if idle_timer.timer.is_finished() {
        let on_special_tile = is_on_special_tile(player_pos, maze_config, floor.0);
        let show_movement_hint = !idle_timer.movement_hint_visible;
        let interaction_visibility = if on_special_tile && !idle_timer.interaction_hint_visible {
            Some(true)
        } else if !on_special_tile && idle_timer.interaction_hint_visible {
            Some(false)
        } else {
            None
        };

        if show_movement_hint {
            idle_timer.movement_hint_visible = true;
        }
        if interaction_visibility.is_some() {
            idle_timer.interaction_hint_visible = true;
        }
        if interaction_visibility == Some(false) {
            idle_timer.interaction_hint_visible = false;
        }

        if show_movement_hint || interaction_visibility.is_some() {
            for (mut visibility, hint) in hint_query.iter_mut() {
                if show_movement_hint && *hint == Hint::Movement {
                    *visibility = Visibility::Visible;
                }
                if *hint == Hint::Interaction {
                    if let Some(visible) = interaction_visibility {
                        *visibility = if visible {
                            Visibility::Visible
                        } else {
                            Visibility::Hidden
                        };
                    }
                }
            }
        }
    }
}

fn hide_all_hints(mut hint_query: Query<(&mut Visibility, &Hint)>, idle_timer: &mut IdleTimer) {
    for (mut visibility, _) in hint_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    idle_timer.hide_all();
}

fn is_on_special_tile(player_pos: &Hex, maze_config: &MazeConfig, floor: u8) -> bool {
    (*player_pos == maze_config.start_pos && floor != 1) || *player_pos == maze_config.end_pos
}
