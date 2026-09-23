use bevy::prelude::*;

use crate::powerups::components::{Cooldown, Duration, IsActive, Kind, Powerup};

pub fn handle_activation(
    mut query: Query<(&Kind, &mut IsActive, &mut Cooldown, Option<&mut Duration>), With<Powerup>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (kind, mut is_active, mut cooldown, mut duration) in query.iter_mut() {
        cooldown.tick(time.delta());

        if let Some(duration) = &mut duration {
            if **is_active {
                duration.tick(time.delta());
                if duration.is_finished() {
                    is_active.deactivate();
                    cooldown.reset();
                }
            }
        }

        let mut try_activate = |key: KeyCode| {
            if cooldown.is_finished() && input.just_pressed(key) {
                is_active.activate();
                if let Some(duration) = &mut duration {
                    duration.reset();
                } else {
                    is_active.deactivate();
                    cooldown.reset();
                }
            }
        };

        match kind {
            Kind::WallJump => try_activate(KeyCode::Space),
            Kind::PathFinder => try_activate(KeyCode::KeyF),
        }

        if cooldown.is_finished() {
            is_active.deactivate();
        }
    }
}

pub fn monitor(query: Query<(&Kind, &IsActive, &Cooldown), With<Powerup>>) {
    for (kind, is_active, cooldown) in query.iter() {
        if **is_active {
            info!("{:?}, {:?}, {:?}", *kind, **is_active, **cooldown);
        }
    }
}
