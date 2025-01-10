use bevy::prelude::*;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct InteractionAssets {
    #[dependency]
    pub(super) hover: Handle<AudioSource>,
    #[dependency]
    pub(super) press: Handle<AudioSource>,
}

impl InteractionAssets {
    pub const PATH_BUTTON_HOVER: &'static str = "audio/sound_effects/button_hover.ogg";
    pub const PATH_BUTTON_PRESS: &'static str = "audio/sound_effects/button_press.ogg";
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load(Self::PATH_BUTTON_HOVER),
            press: assets.load(Self::PATH_BUTTON_PRESS),
        }
    }
}
