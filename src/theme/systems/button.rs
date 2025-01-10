use bevy::prelude::*;

use crate::{
    audio::SoundEffect,
    theme::{assets::InteractionAssets, events::OnPress, prelude::InteractionPalette},
};

pub fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
        }
    }
}

pub fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

pub fn trigger_interaction_sound_effect(
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    interaction_assets: Res<InteractionAssets>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        let source = match interaction {
            Interaction::Hovered => interaction_assets.hover.clone(),
            Interaction::Pressed => interaction_assets.press.clone(),
            _ => continue,
        };
        commands.spawn((
            AudioPlayer::<AudioSource>(source),
            PlaybackSettings::DESPAWN,
            SoundEffect,
        ));
    }
}
