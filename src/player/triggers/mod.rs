mod despawn;
mod respawn;
mod spawn;

use bevy::prelude::*;
use despawn::despawn_players;
use respawn::respawn_player;
use spawn::spawn_player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_player)
        .add_observer(respawn_player)
        .add_observer(despawn_players);
}
