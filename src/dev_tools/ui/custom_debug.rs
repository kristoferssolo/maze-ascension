use crate::{
    floor::components::{CurrentFloor, Floor},
    maze::{commands::RespawnMaze, components::MazeConfig, GlobalMazeConfig},
    player::commands::RespawnPlayer,
    powerups::{pathfinder::Pathfinder, wall_jump::WallJump},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, emath::Numeric, DragValue, TextEdit, Ui},
    EguiContext, PrimaryEguiContext,
};
use hexx::{Hex, HexOrientation};
use rand::{rng, RngExt};
use std::ops::RangeInclusive;

pub fn custom_debug_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    let current_floor = world
        .query_filtered::<(&MazeConfig, &Floor), With<CurrentFloor>>()
        .single(world)
        .ok()
        .map(|(config, floor)| (config.clone(), floor.0));

    egui::Window::new("Custom Debug")
        .default_pos(egui::pos2(400.0, 16.0))
        .show(egui_context.get_mut(), |ui| {
            if let Some((mut maze_config, floor_value)) = current_floor {
                if let Some(mut global_config) = world.get_resource_mut::<GlobalMazeConfig>() {
                    let mut changed = false;
                    ui.heading("Maze Configuration");

                    // Display current floor as non-editable text
                    ui.horizontal(|ui| {
                        ui.label("Current floor:");
                        let mut floor_text = floor_value.to_string();
                        ui.add_enabled(
                            false,
                            TextEdit::singleline(&mut floor_text).desired_width(10.),
                        );
                    });

                    changed |= add_seed_control(ui, &mut maze_config.seed);
                    changed |= add_drag_value_control(
                        ui,
                        "Radius:",
                        &mut maze_config.radius,
                        1.0,
                        1..=100,
                    );
                    changed |= add_drag_value_control(
                        ui,
                        "Height:",
                        &mut global_config.height,
                        0.5,
                        1.0..=50.0,
                    );
                    changed |= add_drag_value_control(
                        ui,
                        "Hex Size:",
                        &mut global_config.hex_size,
                        1.0,
                        1.0..=100.0,
                    );
                    changed |= add_orientation_control(ui, &mut maze_config.layout.orientation);
                    changed |=
                        add_position_control(ui, "Start Position:", &mut maze_config.start_pos);
                    changed |= add_position_control(ui, "End Position:", &mut maze_config.end_pos);

                    // Handle updates
                    if changed {
                        maze_config.update(&global_config);
                        RespawnMaze {
                            floor: floor_value,
                            config: maze_config,
                        }
                        .apply(world);
                        RespawnPlayer.apply(world);
                    }
                }
            }
            ui.separator();
            ui.heading("Powerups");
            if let Some(mut wall_jump) = world.get_resource_mut::<WallJump>() {
                ui.horizontal(|ui| {
                    ui.label(format!("Wall Jump: {}", wall_jump.charges()));
                    if ui.button("Grant Wall Jump").clicked() {
                        wall_jump.grant();
                    }
                });
            }
            if let Some(mut pathfinder) = world.get_resource_mut::<Pathfinder>() {
                ui.horizontal(|ui| {
                    ui.label(format!("PathFinder: {}", pathfinder.charges()));
                    if ui.button("Grant PathFinder").clicked() {
                        pathfinder.grant();
                    }
                });
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
            *seed = rng().random();
            changed = true;
        }

        // Copy button
        if ui.button("📋").clicked() {
            ui.ctx().copy_text(seed.to_string());
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
