use bevy::prelude::*;

use crate::player::character::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, update_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}

fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let mut camera_transform = camera_query
        .single_mut()
        .expect("There should be only one camera. If this has changed, redo this system.");

    if let Ok(player_transform) = player_query.single() {
        // Center the camera on the player
        camera_transform.translation = player_transform.translation;
    }
}
