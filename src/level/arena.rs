use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    collision_layers::GameLayer,
    enemy::slime::{SlimeAssets, slime},
    player::{
        character::{PlayerAssets, player},
        weapon::{WeaponAssets, weapon},
    },
    screens::Screen,
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    weapon_assets: Res<WeaponAssets>,
    slime_assets: Res<SlimeAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            player(&player_assets, &mut meshes, &mut materials),
            weapon(&weapon_assets)
        ],
    ));

    let slime = || slime(&slime_assets, Vec3::new(200.0, 2000.0, 0.0));

    commands.spawn(slime());
    // commands.spawn(slime());

    commands.spawn((
        Name::new("Platform"),
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(1100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -225.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(1100.0, 100.0),
        CollisionLayers::new(GameLayer::Ground, LayerMask::ALL),
    ));
}
