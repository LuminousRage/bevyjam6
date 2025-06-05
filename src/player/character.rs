use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    collision_layers::{player_hit_boxes, player_hurt_boxes},
    health::{Health, hitbox_prefab, hurtbox_prefab},
    physics::creature::Grounded,
};

use super::{
    configs::{CHARACTER_GRAVITY_SCALE, CHARACTER_HEALTH},
    movement::CharacterControllerBundle,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
    app.add_systems(Update, player_fall_recovery);
    app.add_systems(Update, reset_player_gravity_scale);
}

#[derive(Component)]
pub struct Player {
    pub face_direction: Vec2,
    pub attack_direction: Vec2,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub player: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("images/player.png"),
        }
    }
}

fn reset_player_gravity_scale(
    mut player: Single<(&mut GravityScale, Has<Grounded>), With<Player>>,
) {
    let (gs, is_grounded) = &mut *player;

    if *is_grounded {
        gs.0 = CHARACTER_GRAVITY_SCALE;
    }
}

fn player_fall_recovery(
    mut player: Single<(&mut Transform, &mut LinearVelocity, &mut GravityScale), With<Player>>,
) {
    let (transform, lv, gs) = &mut *player;

    if transform.translation.y < -1500.0 {
        lv.y = 0.0;
        gs.0 = 0.5;
        // TODO: add a period of invulnerability
        transform.translation.y = 300.0;
        transform.translation.x = 0.0;
    }
}

pub fn player(
    player_assets: &PlayerAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    (
        Name::new("Player"),
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        Player {
            face_direction: Vec2::X,
            attack_direction: Vec2::X,
        },
        // Sprite {
        //     image: player_assets.player.clone(),
        //     ..default()
        // },
        Mesh2d(meshes.add(Capsule2d::new(40.0, 70.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
        CharacterControllerBundle::new(Collider::capsule(40.0, 70.0)),
        Health::new(CHARACTER_HEALTH),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(CHARACTER_GRAVITY_SCALE),
        children![hurtbox_prefab(
            Collider::capsule(40.0, 70.0),
            player_hurt_boxes(),
            0.5
        )],
    )
}
