use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, screens::Screen};

use super::movement::CharacterControllerBundle;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
    app.insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    player: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("images/player.png"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            player(&player_assets),
            // (
            //     Name::new("Player"),
            //     Collider::capsule(12.5, 20.0),
            //     Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
            //     Sprite {
            //         image: player_assets.player.clone(),
            //         ..default()
            //     },
            //     Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            //     Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            //     ColliderDensity(2.0),
            //     GravityScale(1.5),
            //     RigidBody::Dynamic,
            // )
        ],
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

fn player(player_assets: &PlayerAssets) -> impl Bundle {
    (
        Name::new("Player"),
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        Sprite {
            image: player_assets.player.clone(),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(
            1250.0,
            0.92,
            400.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.5),
    )
}
