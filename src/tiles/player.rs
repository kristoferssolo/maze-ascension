use bevy::{
    color::palettes::css::BLUE,
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use hexx::{Hex, HexLayout, HexOrientation};

use crate::screens::Screen;

use super::tile::{GridSettings, HexDirection, Tile};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    // app.add_systems(Update, move_player);
}

#[derive(Debug)]
pub struct SpawnPlayer;

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once(spawn_player);
    }
}

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Player {
    position: Hex,
}

fn spawn_player(mut commands: Commands, grid_settings: Res<GridSettings>) {
    let starting_hex = Hex::ZERO;
    let layout = HexLayout {
        orientation: HexOrientation::Pointy,
        origin: Vec2::ZERO,
        hex_size: grid_settings.hex_size,
        ..default()
    };

    let world_pos = layout.hex_to_world_pos(starting_hex);

    commands.spawn((
        Name::new("Player"),
        SpriteBundle {
            sprite: Sprite {
                color: BLUE.into(),
                custom_size: Some(grid_settings.hex_size * 0.8),
                ..default()
            },
            transform: Transform::from_translation(world_pos.extend(1.)),
            ..default()
        },
        Player {
            position: starting_hex,
        },
        StateScoped(Screen::Gameplay),
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    grid_settings: Res<GridSettings>,
    mut player_query: Query<&mut Player>,
    tile_query: Query<&Tile>,
) {
    let mut player = player_query.single_mut();

    if let Some(direction) = get_move_direction(&input) {
        let current_tile = tile_query
            .iter()
            .find(|tile| tile.position == player.position)
            .unwrap();
        if current_tile.has_wall(&direction) {
            return;
        }

        let hexx_direction = direction.to_hexx_direction();
        player.position = player.position + hexx_direction;

        let layout = HexLayout {
            orientation: HexOrientation::Pointy,
            origin: Vec2::ZERO,
            hex_size: grid_settings.hex_size,
            ..default()
        };

        let world_pos = layout.hex_to_world_pos(player.position);
    }
}

fn get_move_direction(input: &Res<ButtonInput<KeyCode>>) -> Option<HexDirection> {
    if input.just_pressed(KeyCode::KeyW) {
        return Some(HexDirection::Top);
    }
    if input.just_pressed(KeyCode::KeyE) {
        return Some(HexDirection::TopRight);
    }
    if input.just_pressed(KeyCode::KeyD) {
        return Some(HexDirection::BottomRight);
    }
    if input.just_pressed(KeyCode::KeyS) {
        return Some(HexDirection::Bottom);
    }
    if input.just_pressed(KeyCode::KeyA) {
        return Some(HexDirection::BottomLeft);
    }
    if input.just_pressed(KeyCode::KeyQ) {
        return Some(HexDirection::TopLeft);
    }
    None
}
