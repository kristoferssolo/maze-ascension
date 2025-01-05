use bevy::prelude::*;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
pub struct HintAssets {
    #[dependency]
    pub arrows: Handle<Image>,
    #[dependency]
    pub interaction: Handle<Image>,
}

impl HintAssets {
    pub const PATH_ARROWS: &str = "images/hints/arrows.png";
    pub const PATH_INTERACTION: &str = "images/hints/interaction.png";
}

impl FromWorld for HintAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            arrows: assets.load(Self::PATH_ARROWS),
            interaction: assets.load(Self::PATH_INTERACTION),
        }
    }
}
