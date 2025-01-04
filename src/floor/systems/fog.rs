use bevy::prelude::*;

use crate::theme::{palette::rose_pine::RosePineDawn, prelude::ColorScheme};

pub fn setup_camera_fog(mut commands: Commands) {
    commands.spawn((
        Name::new("Fog"),
        DistanceFog {
            color: RosePineDawn::Overlay.to_color(),
            directional_light_color: RosePineDawn::Overlay.to_color(),
            falloff: FogFalloff::Linear {
                start: 1.,
                end: 20.,
            },
            ..default()
        },
    ));
}
