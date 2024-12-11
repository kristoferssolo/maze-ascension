//! Helper traits for creating common widgets.

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::theme::{interaction::InteractionPalette, palette::*};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: SpawnUi> Widgets for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn_ui((
            Name::new("Button"),
            Button,
            ImageNode {
                color: NODE_BACKGROUND,
                ..default()
            },
            Node {
                width: Px(200.0),
                height: Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children.spawn_ui((
                Name::new("Button Text"),
                Text(text.into()),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(BUTTON_TEXT),
            ));
        });

        entity
    }

    fn header(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn_ui((
            Name::new("Header"),
            Node {
                width: Px(500.0),
                height: Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NODE_BACKGROUND),
        ));
        entity.with_children(|children| {
            children.spawn_ui((
                Name::new("Header Text"),
                Text(text.into()),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(HEADER_TEXT),
            ));
        });
        entity
    }

    fn label(&mut self, _text: impl Into<String>) -> EntityCommands {
        let entity = self.spawn_ui((
            Name::new("Label"),
            Text::default(),
            // TextBundle::from_section(
            //     text,
            //     TextStyle {
            //         font_size: 24.0,
            //         color: LABEL_TEXT,
            //         ..default()
            //     },
            // )
            // .with_style(Style {
            //     width: Px(500.0),
            //     ..default()
            // }),
        ));
        entity
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait SpawnUi {
    fn spawn_ui<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl SpawnUi for Commands<'_, '_> {
    fn spawn_ui<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl SpawnUi for ChildBuilder<'_> {
    fn spawn_ui<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
