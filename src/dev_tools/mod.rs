mod ui;

use crate::screens::Screen;
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::custom_debug_ui;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, log_transitions::<Screen>)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(register_custom_debug_ui)
        .add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
}

fn register_custom_debug_ui(app: &mut App) {
    app.add_systems(EguiPrimaryContextPass, custom_debug_ui);
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<bevy::ui_render::GlobalUiDebugOptions>) {
    options.toggle();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::window::PrimaryWindow;
    use bevy_egui::EguiContext;

    #[test]
    fn custom_debug_draws_in_egui_primary_pass() {
        let mut app = App::new();
        register_custom_debug_ui(&mut app);
        let context = EguiContext::default();
        let mut frame = context.clone();
        app.world_mut().spawn((PrimaryWindow, context));

        frame.get_mut().begin_pass(Default::default());
        let _ = app
            .world_mut()
            .try_run_schedule(bevy_egui::EguiPrimaryContextPass);
        let output = frame.get_mut().end_pass();

        assert!(
            !output.shapes.is_empty(),
            "Custom Debug window was not drawn"
        );
    }
}
