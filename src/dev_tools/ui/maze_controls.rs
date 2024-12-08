use std::ops::RangeInclusive;

use bevy::{prelude::*, window::PrimaryWindow};
use hexx::{Hex, HexOrientation};
use rand::{thread_rng, Rng};

use crate::maze::{events::RecreateMazeEvent, MazeConfig, MazePluginLoaded};
use bevy_egui::{
    egui::{self, emath::Numeric, DragValue, TextEdit, Ui},
    EguiContext,
};

pub(crate) fn maze_controls_ui(world: &mut World) {
    if world.get_resource::<MazePluginLoaded>().is_none() {
        return;
    }

    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };

    let mut egui_context = egui_context.clone();

    egui::Window::new("Maze Controls").show(egui_context.get_mut(), |ui| {
        if let Some(mut maze_config) = world.get_resource_mut::<MazeConfig>() {
            let mut changed = false;
            ui.heading("Maze Configuration");

            changed |= add_seed_control(ui, &mut maze_config.seed);

            changed |= add_drag_value_control(ui, "Radius:", &mut maze_config.radius, 1.0, 1..=100);
            changed |=
                add_drag_value_control(ui, "Height:", &mut maze_config.height, 0.5, 1.0..=50.0);
            changed |= add_drag_value_control(
                ui,
                "Hex Size:",
                &mut maze_config.hex_size,
                1.0,
                1.0..=100.0,
            );

            changed |= add_orientation_control(ui, &mut maze_config.layout.orientation);

            changed |= add_position_control(ui, "Start Position:", &mut maze_config.start_pos);
            changed |= add_position_control(ui, "End Position:", &mut maze_config.end_pos);

            // Trigger recreation if any value changed
            if changed {
                maze_config.update();
                if let Some(mut event_writer) =
                    world.get_resource_mut::<Events<RecreateMazeEvent>>()
                {
                    event_writer.send(RecreateMazeEvent { floor: 1 });
                }
            }
        }
    });
}

fn add_drag_value_control<T: Numeric>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    speed: f64,
    range: RangeInclusive<T>,
) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label(label);
        let response = ui.add(DragValue::new(value).speed(speed).range(range));
        changed = response.changed();
    });
    changed
}

fn add_position_control(ui: &mut Ui, label: &str, pos: &mut Hex) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label(label);
        let response_x = ui.add(DragValue::new(&mut pos.x).speed(1).prefix("x: "));
        let response_y = ui.add(DragValue::new(&mut pos.y).speed(1).prefix("y: "));
        changed = response_x.changed() || response_y.changed();
    });
    changed
}

fn add_seed_control(ui: &mut Ui, seed: &mut u64) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Seed:");

        let mut seed_text = seed.to_string();

        let response = ui.add(
            TextEdit::singleline(&mut seed_text)
                .desired_width(150.0)
                .hint_text("Enter seed"),
        );

        // Parse text input when changed
        if response.changed() {
            if let Ok(new_seed) = seed_text.parse::<u64>() {
                *seed = new_seed;
                changed = true;
            }
        }

        // New random seed button
        if ui.button("🎲").clicked() {
            *seed = thread_rng().gen();
            changed = true;
        }

        // Copy button
        if ui.button("📋").clicked() {
            ui.output_mut(|o| o.copied_text = seed.to_string());
        }
    });

    changed
}

fn add_orientation_control(ui: &mut Ui, orientation: &mut HexOrientation) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Orientation:");

        let response = ui.radio_value(orientation, HexOrientation::Flat, "Flat");
        changed |= response.changed();

        let response = ui.radio_value(orientation, HexOrientation::Pointy, "Pointy");
        changed |= response.changed();
    });

    changed
}
