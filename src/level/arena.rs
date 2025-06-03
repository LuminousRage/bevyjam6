use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    player::character::{PlayerAssets, player},
    screens::Screen,
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![player(&player_assets),],
    ));

    commands.spawn((
        Name::new("Obj"),
        Mesh2d(meshes.add(Capsule2d::new(12.5, 20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
        Collider::capsule(12.5, 20.0),
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.5),
        RigidBody::Dynamic,
    ));

    commands.spawn((
        Name::new("Platform"),
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(1100.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -175.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(1100.0, 50.0),
    ));
}
