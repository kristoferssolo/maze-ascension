use bevy::prelude::*;

use crate::hint::{
    assets::HintAssets,
    components::{Hint, IdleTimer},
};

pub fn setup(mut commands: Commands, hint_assets: Res<HintAssets>) {
    commands.spawn((
        Name::new("Movement hint"),
        Hint::Movement,
        Visibility::Hidden,
    ));

    commands.spawn((
        Name::new("Interaction hint"),
        Hint::Interaction,
        Visibility::Hidden,
    ));

    // Add idle timer
    commands.spawn(IdleTimer::default());
}
