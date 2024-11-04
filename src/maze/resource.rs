use bevy::prelude::*;
use hexx::{Hex, HexLayout, HexOrientation};
use rand::{thread_rng, Rng};

pub(crate) const HEX_SIZE: f32 = 6.;

#[derive(Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct MazeConfig {
    pub radius: u32,
    pub height: f32,
    pub start_pos: Hex,
    pub end_pos: Hex,
}

impl Default for MazeConfig {
    fn default() -> Self {
        let mut rng = thread_rng();
        let radius = 11;
        let start_pos = Hex::new(
            rng.gen_range(-radius..radius),
            rng.gen_range(-radius..radius),
        );
        let end_pos = Hex::new(
            rng.gen_range(-radius..radius),
            rng.gen_range(-radius..radius),
        );
        debug!("Start pos: ({},{})", start_pos.x, start_pos.y);
        debug!("End pos: ({},{})", end_pos.x, end_pos.y);
        Self {
            radius: radius as u32,
            height: 20.,
            start_pos,
            end_pos,
        }
    }
}

#[derive(Debug, Reflect, Resource, Deref, DerefMut, Clone)]
#[reflect(Resource)]
pub struct Layout(pub HexLayout);

impl FromWorld for Layout {
    fn from_world(_world: &mut World) -> Self {
        Self(HexLayout {
            orientation: HexOrientation::Pointy,
            hex_size: Vec2::splat(HEX_SIZE),
            ..default()
        })
    }
}
