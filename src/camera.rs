use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::constants::{BASE_ZOOM_SPEED, DISTANCE_SCALE_FACTOR, MAX_ZOOM, MIN_ZOOM};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, camera_zoom);
}

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(200., 200., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}

fn camera_zoom(
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut scrool_evr: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    let current_distance = transform.translation.length();

    // Calculate zoom speed based on distance
    let distance_multiplier = (current_distance / MIN_ZOOM).powf(DISTANCE_SCALE_FACTOR);
    let adjusted_zoom_speed = BASE_ZOOM_SPEED * distance_multiplier;

    let mut zoom_delta = 0.0;

    if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        zoom_delta += adjusted_zoom_speed * time.delta_secs() * 25.;
    }

    if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        zoom_delta -= adjusted_zoom_speed * time.delta_secs() * 25.;
    }

    for ev in scrool_evr.read() {
        zoom_delta += ev.y * adjusted_zoom_speed;
    }

    if zoom_delta != 0.0 {
        let forward = transform.translation.normalize();
        let new_distance = (current_distance - zoom_delta).clamp(MIN_ZOOM, MAX_ZOOM);
        transform.translation = forward * new_distance;
    }
}
