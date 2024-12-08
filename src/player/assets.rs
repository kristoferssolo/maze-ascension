use crate::theme::palette::rose_pine::PINE;
use bevy::prelude::*;

pub(super) fn generate_pill_mesh(radius: f32, half_length: f32) -> Mesh {
    Mesh::from(Capsule3d {
        radius,
        half_length,
    })
}

pub(super) fn blue_material() -> StandardMaterial {
    StandardMaterial {
        base_color: PINE,
        emissive: PINE.to_linear() * 3.,
        ..default()
    }
}
