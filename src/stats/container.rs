use bevy::prelude::*;

pub trait StatsContainer {
    fn ui_stats(&mut self) -> EntityCommands;
}

impl StatsContainer for Commands<'_, '_> {
    fn ui_stats(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Stats Root"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.),
                right: Val::Px(10.),
                row_gap: Val::Px(8.),
                align_items: AlignItems::End,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
    }
}
