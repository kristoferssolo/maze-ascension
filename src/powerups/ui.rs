use super::{pathfinder::Pathfinder, wall_jump::WallJump};
use crate::{
    screens::{GameplayElement, Screen},
    theme::{palette::rose_pine::RosePineDawn, prelude::ColorScheme},
};
use bevy::prelude::*;

#[derive(Component)]
struct PowerupHud;

#[derive(Component)]
enum AbilityStatus {
    WallJump,
    Route,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_hud)
        .add_systems(Update, update_hud.run_if(in_state(Screen::Gameplay)));
}

fn spawn_hud(mut commands: Commands, existing: Query<Entity, With<PowerupHud>>) {
    if !existing.is_empty() {
        return;
    }

    commands
        .spawn((
            Name::new("Powerup controls"),
            PowerupHud,
            GameplayElement,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                bottom: Val::Px(16.0),
                width: Val::Px(360.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(RosePineDawn::Surface.to_color().with_alpha(0.9)),
        ))
        .with_children(|parent| {
            spawn_row(
                parent,
                "Space + move",
                "Cross wall",
                AbilityStatus::WallJump,
            );
            spawn_row(parent, "F", "Show route", AbilityStatus::Route);
        });
}

fn spawn_row(
    parent: &mut ChildSpawnerCommands,
    key: &'static str,
    action: &'static str,
    status: AbilityStatus,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(key),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(RosePineDawn::Subtle.to_color()),
            ));
            row.spawn((
                Text::new(action),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(RosePineDawn::Text.to_color()),
            ));
            row.spawn((
                Text::new("Ready"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(RosePineDawn::Gold.to_color()),
                status,
            ));
        });
}

fn update_hud(
    wall_jump: Res<WallJump>,
    pathfinder: Res<Pathfinder>,
    mut statuses: Query<(&AbilityStatus, &mut Text, &mut TextColor)>,
) {
    for (ability, mut text, mut color) in &mut statuses {
        let remaining = match ability {
            AbilityStatus::WallJump => wall_jump.cooldown_remaining_secs(),
            AbilityStatus::Route => pathfinder.cooldown_remaining_secs(),
        };
        let (label, tint) = remaining.map_or_else(
            || ("Ready".to_owned(), RosePineDawn::Gold.to_color()),
            |seconds| {
                (
                    format!("{}s", seconds.ceil() as u32),
                    RosePineDawn::Muted.to_color(),
                )
            },
        );
        if text.0 != label {
            text.0 = label;
        }
        if color.0 != tint {
            color.0 = tint;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_some;

    #[test]
    fn hud_shows_cooldown_and_ready_state() {
        let mut app = App::new();
        app.init_resource::<WallJump>();
        app.init_resource::<Pathfinder>();
        app.add_systems(Update, update_hud);
        app.world_mut().spawn((
            AbilityStatus::WallJump,
            Text::new("Ready"),
            TextColor::default(),
        ));
        app.world_mut().spawn((
            AbilityStatus::Route,
            Text::new("Ready"),
            TextColor::default(),
        ));
        assert_some!(app.world_mut().get_resource_mut::<WallJump>()).consume();

        app.update();

        let mut statuses = app.world_mut().query::<(&AbilityStatus, &Text)>();
        let labels = statuses
            .iter(app.world())
            .map(|(ability, text)| match ability {
                AbilityStatus::WallJump => ("wall", text.0.as_str()),
                AbilityStatus::Route => ("route", text.0.as_str()),
            })
            .collect::<Vec<_>>();
        assert!(labels.contains(&("wall", "10s")));
        assert!(labels.contains(&("route", "Ready")));
    }

    #[test]
    fn hud_is_not_duplicated_when_gameplay_resumes() {
        let mut app = App::new();
        app.add_systems(Update, spawn_hud);

        app.update();
        app.update();

        let world = app.world_mut();
        let mut roots = world.query_filtered::<Entity, With<PowerupHud>>();
        assert_eq!(roots.iter(world).count(), 1);
    }
}
