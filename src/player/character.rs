use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, health::Health};

use super::{
    configs::{CHARACTER_GRAVITY_SCALE, CHARACTER_HEALTH},
    movement::CharacterControllerBundle,
    weapon::weapon,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
}

#[derive(Component)]
pub struct Player;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub player: Handle<Image>,
    #[dependency]
    pub weapon: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("images/player.png"),
            weapon: assets.load("images/weapon.png"),
        }
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
        Player,
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
    )
}
