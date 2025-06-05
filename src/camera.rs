use bevy::prelude::*;

use crate::player::character::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, update_camera);
}

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 2.;
const FLOOR_MIN_Y: f32 = 100.0;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}

fn update_camera(
    camera_query: Single<&mut Transform, With<Camera2d>>,
    player_query: Option<Single<&Transform, (With<Player>, Without<Camera2d>)>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query;

    if let Some(player_transform) = player_query {
        // TODO: Add track player when player falls off
        let Vec3 { x, y, .. } = player_transform.translation;
        let direction = Vec3::new(x, y.max(FLOOR_MIN_Y), camera_transform.translation.z);

        camera_transform
            .translation
            .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
    }
}
