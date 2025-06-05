use bevy::{prelude::*, sprite::Anchor};

use crate::{asset_tracking::LoadResource, player::attack::Attack};

use super::{character::Player, movement::PlayerFaceDirection};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WeaponAssets>();
    app.load_resource::<WeaponAssets>();
    app.add_systems(Update, move_weapon_while_idle);
}

// offset in pixels to line up the weapon
const OFFSET_FROM_BASE: u64 = 898;
const OFFSET_FROM_EXTEND: u64 = 178;
const WEAPON_FOLLOW_OFFSET: Vec3 = Vec3::new(45.0, -35.0, -1.0);

#[derive(Component)]
pub struct Weapon;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WeaponAssets {
    #[dependency]
    pub weapon_base: Handle<Image>,
    pub weapon_extend: Handle<Image>,
    pub weapon_head: Handle<Image>,
}

impl FromWorld for WeaponAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            weapon_base: assets.load("images/weapon_base.png"),
            weapon_extend: assets.load("images/weapon_extend.png"),
            weapon_head: assets.load("images/weapon_head.png"),
        }
    }
}

fn update_weapon_length(mut weapon: Single<(&Children), With<Weapon>>) {}

pub fn weapon(player_assets: &WeaponAssets) -> impl Bundle {
    (
        Name::new("Weapon"),
        Weapon,
        Transform {
            scale: Vec2::splat(0.055).extend(1.0),
            ..default()
        },
        Visibility::default(),
        children![
            (
                Name::new("Base"),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Sprite {
                    image: player_assets.weapon_base.clone(),
                    anchor: Anchor::BottomCenter,
                    ..default()
                }
            ),
            (
                Name::new("Extend"),
                Transform::from_xyz(0.0, OFFSET_FROM_BASE as f32, 0.0),
                Sprite {
                    image: player_assets.weapon_extend.clone(),
                    anchor: Anchor::BottomCenter,
                    ..default()
                }
            ),
            (
                Name::new("Head"),
                Transform::from_xyz(0.0, (OFFSET_FROM_EXTEND + OFFSET_FROM_BASE) as f32, 0.0),
                Sprite {
                    image: player_assets.weapon_head.clone(),
                    anchor: Anchor::BottomCenter,
                    ..default()
                }
            )
        ],
    )
}

fn move_weapon_while_idle(
    mut following: Single<&mut Transform, With<Weapon>>,
    player: Option<
        Single<
            (&Transform, &PlayerFaceDirection),
            (With<Player>, Without<Weapon>, Without<Attack>),
        >,
    >,
    time: Res<Time>,
) {
    if let Some(p) = player {
        let (transform, face_direction) = *p;
        let delta_time = time.delta_secs();
        let Vec3 { x, y, z } = following.scale;
        let direction = if following.translation.x > transform.translation.x {
            1.0
        } else {
            -1.0
        };
        following.scale = Vec3::new(direction * x.abs(), y, z);
        let target_translation =
            &transform.translation + WEAPON_FOLLOW_OFFSET * (Vec3::new(-face_direction.0, 1., 1.));
        following
            .translation
            .smooth_nudge(&target_translation, 2.0, delta_time);
    }
}
