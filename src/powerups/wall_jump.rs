use crate::{screens::Screen, AppSystems};
use bevy::prelude::*;

const COOLDOWN_SECONDS: f32 = 10.0;

#[derive(Debug, Default, Resource)]
pub struct WallJump {
    cooldown: Option<Timer>,
}

impl WallJump {
    pub const fn is_ready(&self) -> bool {
        self.cooldown.is_none()
    }

    pub fn cooldown_remaining_secs(&self) -> Option<f32> {
        self.cooldown.as_ref().map(Timer::remaining_secs)
    }

    pub fn consume(&mut self) {
        self.cooldown = Some(Timer::from_seconds(COOLDOWN_SECONDS, TimerMode::Once));
    }

    fn tick(&mut self, delta: std::time::Duration) {
        if self
            .cooldown
            .as_mut()
            .is_some_and(|timer| timer.tick(delta).is_finished())
        {
            self.cooldown = None;
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WallJump>()
        .add_systems(OnEnter(Screen::Title), reset)
        .add_systems(
            Update,
            tick.in_set(AppSystems::TickTimers)
                .run_if(in_state(Screen::Gameplay)),
        );
}

fn reset(mut wall_jump: ResMut<WallJump>) {
    *wall_jump = WallJump::default();
}

fn tick(time: Res<Time>, mut wall_jump: ResMut<WallJump>) {
    wall_jump.tick(time.delta());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooldown_starts_only_when_used_and_expires() {
        let mut wall_jump = WallJump::default();
        assert!(wall_jump.is_ready());

        wall_jump.consume();
        assert!(!wall_jump.is_ready());
        wall_jump.tick(std::time::Duration::from_secs(9));
        assert!(!wall_jump.is_ready());
        wall_jump.tick(std::time::Duration::from_secs(1));
        assert!(wall_jump.is_ready());
    }
}
