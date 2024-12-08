use crate::screens::Screen;
use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

use super::ui::{inspector_ui, maze_controls_ui};

#[derive(Debug)]
pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_transitions::<Screen>)
            .add_plugins(EguiPlugin)
            .add_plugins(DebugUiPlugin)
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_systems(Update, (inspector_ui, maze_controls_ui))
            .add_systems(
                Update,
                toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
            );
    }
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
