//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.

pub mod assets;
mod colorscheme;
pub mod components;
pub mod events;
pub mod palette;
mod systems;
mod widgets;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{
        colorscheme::{ColorScheme, ColorSchemeWrapper},
        components::{InteractionPalette, UrlLink},
        events::OnPress,
        palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}

use assets::InteractionAssets;
use bevy::prelude::*;
use prelude::InteractionPalette;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.load_resource::<InteractionAssets>();
    app.add_plugins(systems::plugin);
}
