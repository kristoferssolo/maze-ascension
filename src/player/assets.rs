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

#[derive(Resource, Asset, Reflect, Clone)]
pub struct PlayerAssets {
    // This #[dependency] attribute marks the field as a dependency of the Asset.
    // This means that it will not finish loading until the labeled asset is also loaded.
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl PlayerAssets {
    pub const PATH_STEP_1: &str = "audio/sound_effects/step1.ogg";
    pub const PATH_STEP_2: &str = "audio/sound_effects/step2.ogg";
    pub const PATH_STEP_3: &str = "audio/sound_effects/step3.ogg";
    pub const PATH_STEP_4: &str = "audio/sound_effects/step4.ogg";
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            steps: vec![
                assets.load(Self::PATH_STEP_1),
                assets.load(Self::PATH_STEP_2),
                assets.load(Self::PATH_STEP_3),
                assets.load(Self::PATH_STEP_4),
            ],
        }
    }
}
