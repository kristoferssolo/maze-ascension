use bevy::{color::palettes::css::BLACK, prelude::*};

use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    path::PathBuilder,
    plugin::ShapePlugin,
};
use rand::{thread_rng, Rng};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ShapePlugin);
    app.add_systems(Startup, setup_system);
}
