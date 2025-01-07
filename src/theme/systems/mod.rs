mod button;

use bevy::prelude::*;
use button::{apply_interaction_palette, trigger_interaction_sound_effect, trigger_on_press};

use super::assets::InteractionAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            trigger_on_press,
            apply_interaction_palette,
            trigger_interaction_sound_effect,
        )
            .run_if(resource_exists::<InteractionAssets>),
    );
}
