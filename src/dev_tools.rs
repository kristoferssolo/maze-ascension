//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::PrimaryWindow,
};

use bevy_inspector_egui::{bevy_egui::EguiContext, DefaultInspectorConfigPlugin};

use crate::{maze::events::RecreateMazeEvent, screens::Screen};
use bevy_egui::{
    egui::{self, Button, Color32, ScrollArea},
    EguiPlugin,
};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>)
        .add_plugins(EguiPlugin)
        .add_plugins(DebugUiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_systems(Update, inspector_ui)
        // Toggle the debug overlay for UI.
        .add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };

    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
        });

        ui.add_space(8.);

        let button = Button::new("Recreate maze").fill(Color32::from_rgb(108, 108, 108));

        if ui.add(button).clicked() {
            if let Some(mut event_writer) = world.get_resource_mut::<Events<RecreateMazeEvent>>() {
                event_writer.send(RecreateMazeEvent { floor: 1 });
            }
        }
    });
}
