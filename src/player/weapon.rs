use bevy::prelude::*;

use super::{
    character::{Player, PlayerAssets},
    movement::PlayerFaceDirection,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, move_weapon);
}

#[derive(Component)]
pub struct Weapon;

pub fn weapon(
    player_assets: &PlayerAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    (
        Name::new("Weapon"),
        Weapon,
        Transform {
            scale: Vec2::splat(0.055).extend(1.0),
            // translation: Vec3::new(85.0, 120.0, -1.0),
            ..default()
        },
        // Mesh2d(meshes.add(Circle::new(10.0))),
        // MeshMaterial2d(materials.add(Color::srgb(0.9, 0.2, 0.2))),
        Sprite {
            image: player_assets.weapon.clone(),
            ..default()
        },
    )
}

fn move_weapon(
    mut following: Single<&mut Transform, With<Weapon>>,
    target: Single<(&Transform, &PlayerFaceDirection), (With<Player>, Without<Weapon>)>,
    time: Res<Time>,
) {
    let (transform, face_direction) = *target;
    let delta_time = time.delta_secs();
    let Vec3 { x, y, z } = following.scale;
    let direction = if following.translation.x > transform.translation.x {
        1.0
    } else {
        -1.0
    };
    following.scale = Vec3::new(direction * x.abs(), y, z);
    let target_translation =
        &transform.translation + Vec3::new(45.0 * -face_direction.0, 55.0, -1.0);
    following
        .translation
        .smooth_nudge(&target_translation, 2.0, delta_time);
}
