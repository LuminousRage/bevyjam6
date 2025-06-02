use crate::{asset_tracking::LoadResource, components::movement::MovementController};

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
}

#[derive(Bundle)]
pub(super) struct PlayerBundle {
    data: PlayerData,
    sprite: Sprite,
}

#[derive(Component)]
pub(super) struct PlayerData {
    name: &'static str,
    description: &'static str,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    sprite: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            sprite: assets.load("images/player.png"),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

/// The player character.
pub fn player(player_assets: &PlayerAssets) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Sprite {
            image: player_assets.sprite.clone(),
            ..Default::default()
        },
        Transform::from_scale(Vec2::splat(0.0).extend(1.0)),
        MovementController {
            max_speed: 1.0,
            ..default()
        },
    )
}

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn((
        Name::new("Player"),
        PlayerData {
            name: "Brother",
            description: "The player or something",
        },
        Transform::default(),
        Visibility::default(),
        // StateScoped(Screen::Gameplay),
        children![player(&player_assets),],
    ));
}
