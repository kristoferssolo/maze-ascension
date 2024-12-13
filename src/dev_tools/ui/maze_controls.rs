use crate::{
    floor::components::{CurrentFloor, Floor},
    maze::{components::MazeConfig, events::RecreateMazeEvent, GlobalMazeConfig, MazePluginLoaded},
    player::events::RespawnPlayer,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, emath::Numeric, DragValue, TextEdit, Ui},
    EguiContext,
};
use hexx::{Hex, HexOrientation};
use rand::{thread_rng, Rng};
use std::ops::RangeInclusive;

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

    let Ok((mut maze_config, floor)) = world
        .query_filtered::<(&mut MazeConfig, &Floor), With<CurrentFloor>>()
        .get_single(world)
    else {
        return;
    };
    let mut maze_config = (*maze_config).clone();

    egui::Window::new("Maze Controls").show(egui_context.get_mut(), |ui| {
        if let Some(mut global_config) = world.get_resource_mut::<GlobalMazeConfig>() {
            let mut changed = false;
            ui.heading("Maze Configuration");

            changed |= add_seed_control(ui, &mut maze_config.seed);

            changed |= add_drag_value_control(ui, "Radius:", &mut maze_config.radius, 1.0, 1..=100);
            changed |=
                add_drag_value_control(ui, "Height:", &mut global_config.height, 0.5, 1.0..=50.0);
            changed |= add_drag_value_control(
                ui,
                "Hex Size:",
                &mut global_config.hex_size,
                1.0,
                1.0..=100.0,
            );

            changed |= add_orientation_control(ui, &mut maze_config.layout.orientation);

            changed |= add_position_control(ui, "Start Position:", &mut maze_config.start_pos);
            changed |= add_position_control(ui, "End Position:", &mut maze_config.end_pos);

            // Trigger recreation if any value changed
            if changed {
                maze_config.update(&global_config);

                if let Ok(mut actual_maze_config) = world
                    .query_filtered::<&mut MazeConfig, With<CurrentFloor>>()
                    .get_single_mut(world)
                {
                    *actual_maze_config = maze_config;
                }

                if let Some(mut event_writer) =
                    world.get_resource_mut::<Events<RecreateMazeEvent>>()
                {
                    event_writer.send(RecreateMazeEvent { floor: floor.0 });
                }
                if let Some(mut event_writer) = world.get_resource_mut::<Events<RespawnPlayer>>() {
                    event_writer.send(RespawnPlayer);
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
        let response_q = ui.add(DragValue::new(&mut pos.x).speed(1).prefix("q: "));
        let response_r = ui.add(DragValue::new(&mut pos.y).speed(1).prefix("r: "));
        changed = response_r.changed() || response_q.changed();
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
