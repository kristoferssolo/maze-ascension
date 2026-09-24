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
    use claims::assert_ok;

    #[test]
    fn custom_debug_runs_in_egui_primary_pass() {
        let mut app = App::new();
        register_custom_debug_ui(&mut app);
        let registered = assert_ok!(app.world_mut().try_schedule_scope(
            EguiPrimaryContextPass,
            |world, schedule| {
                assert_ok!(schedule.initialize(world));
                assert_ok!(schedule.systems())
                    .any(|(_, system)| system.name().to_string().contains("custom_debug_ui"))
            }
        ));

        assert!(registered);
    }
}
