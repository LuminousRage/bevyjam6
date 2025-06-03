use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, health::Health};

use super::movement::CharacterControllerBundle;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
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

pub fn player(player_assets: &PlayerAssets) -> impl Bundle {
    (
        Name::new("Player"),
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        Sprite {
            image: player_assets.player.clone(),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)),
        Health::new(100.0),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.5),
    )
}
