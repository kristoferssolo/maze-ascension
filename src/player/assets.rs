use bevy::prelude::*;

use crate::theme::{palette::rose_pine::RosePineDawn, prelude::ColorScheme};

pub(super) fn generate_pill_mesh(radius: f32, half_length: f32) -> Mesh {
    Mesh::from(Capsule3d {
        radius,
        half_length,
    })
}

pub(super) fn blue_material() -> StandardMaterial {
    let color = RosePineDawn::Pine;
    StandardMaterial {
        base_color: color.to_color(),
        emissive: color.to_linear_rgba() * 3.,
        ..default()
    }
}
