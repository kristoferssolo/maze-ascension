use bevy::{prelude::*, ui::Val::*};

use crate::hint::{
    assets::HintAssets,
    components::{Hint, IdleTimer},
};

pub fn spawn_hints(mut commands: Commands, hint_assets: Res<HintAssets>) {
    commands.spawn((
        Name::new("Movement hint"),
        Hint::Movement,
        Visibility::Hidden,
        ImageNode {
            image: hint_assets.arrows.clone(),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            right: Px(20.0),
            bottom: Px(20.0),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Interaction hint"),
        Hint::Interaction,
        Visibility::Hidden,
        ImageNode {
            image: hint_assets.interaction.clone(),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            right: Px(20.0),
            bottom: Px(168.0),
            ..default()
        },
    ));

    // Add idle timer
    commands.spawn(IdleTimer::default());
}
