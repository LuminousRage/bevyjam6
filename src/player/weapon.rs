use avian2d::prelude::Collider;
use bevy::{prelude::*, sprite::Anchor};

use crate::{
    asset_tracking::LoadResource, collision_layers::player_hit_boxes, health::hitbox_prefab,
    player::attack::Attack,
};

use super::character::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WeaponAssets>();
    app.load_resource::<WeaponAssets>();
    app.add_systems(Update, (move_weapon, update_weapon_length));
}

// offset in pixels to line up the weapon
const OFFSET_FROM_BASE: u64 = 898;
const OFFSET_FROM_EXTEND: u64 = 178;
const EXTEND_SIZE: u64 = 595;
const WEAPON_FOLLOW_OFFSET: Vec3 = Vec3::new(55.0, -35.0, -1.0);
const WEAPON_ATTACK_HORIZONTAL_OFFSET: Vec3 = Vec3::new(-60.0, -40.0, -1.0);
const INACTIVE_WEAPON_TRANSPARENCY: f32 = 0.4;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct WeaponParts;

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

fn update_weapon_length(
    attack: Option<Single<&Attack>>,
    mut children: Query<(&Name, &mut Transform), With<Sprite>>,
) {
    let extend_scale = if let Some(attack) = attack {
        attack.extend_scale
    } else {
        1.0
    };

    for (name, mut transform) in children.iter_mut() {
        match name.as_str() {
            "Weapon Base" => {
                transform.translation.y = 0.0;
            }
            "Weapon Extend" => {
                transform.translation.y = (OFFSET_FROM_BASE) as f32;
                transform.scale.y = extend_scale;
            }
            "Weapon Head" => {
                let extend_translation =
                    extend_scale * EXTEND_SIZE as f32 - (EXTEND_SIZE - OFFSET_FROM_EXTEND) as f32;

                transform.translation.y = OFFSET_FROM_BASE as f32 + extend_translation - 1.0;
            }
            _ => {}
        }
    }
}

pub fn weapon(player_assets: &WeaponAssets) -> impl Bundle {
    (
        Name::new("Weapon"),
        Weapon,
        Transform {
            scale: Vec2::splat(0.050).extend(1.0),
            ..default()
        },
        Visibility::default(),
        children![
            (
                Name::new("Weapon Base"),
                Transform::from_xyz(0.0, 0.0, 0.0),
                WeaponParts,
                Sprite {
                    image: player_assets.weapon_base.clone(),
                    anchor: Anchor::BottomCenter,
                    ..default()
                }
            ),
            (
                Name::new("Weapon Extend"),
                Transform::from_xyz(0.0, OFFSET_FROM_BASE as f32, 0.0),
                WeaponParts,
                Sprite {
                    image: player_assets.weapon_extend.clone(),
                    anchor: Anchor::BottomCenter,
                    ..default()
                }
            ),
            (
                Name::new("Weapon Head"),
                Transform::from_xyz(0.0, (OFFSET_FROM_EXTEND + OFFSET_FROM_BASE) as f32, 0.0),
                WeaponParts,
                Sprite {
                    image: player_assets.weapon_head.clone(),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                children![hitbox_prefab(
                    Collider::circle(30.0),
                    player_hit_boxes(),
                    0.5,
                    10.0
                )]
            )
        ],
    )
}

fn color_with_transparency(alpha: f32) -> Color {
    Color::srgba(1.0, 1.0, 1.0, alpha)
}

fn move_weapon(
    mut following: Single<&mut Transform, With<Weapon>>,
    mut following_parts: Query<&mut Sprite, With<WeaponParts>>,
    player_without_attack: Option<
        Single<(&Transform, &Player), (Without<Weapon>, Without<Attack>)>,
    >,
    player_with_attack: Option<Single<(&Transform, &Player, &Attack), Without<Weapon>>>,

    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    if let Some(p) = player_without_attack {
        let (transform, player) = *p;
        following_parts.iter_mut().for_each(|mut sprite| {
            sprite.color = color_with_transparency(INACTIVE_WEAPON_TRANSPARENCY);
        });
        // following_sprite.color = color_with_transparency(INACTIVE_WEAPON_TRANSPARENCY);
        following.scale = {
            let Vec3 { x, y, z } = following.scale;
            let direction = if following.translation.x > transform.translation.x {
                1.0
            } else {
                -1.0
            };
            Vec3::new(direction * x.abs(), y, z)
        };
        following.rotation = Quat::default();
        let target_translation = &transform.translation
            + WEAPON_FOLLOW_OFFSET * (Vec3::new(-player.face_direction.x, 1., 1.));
        following
            .translation
            .smooth_nudge(&target_translation, 2.0, delta_time);
    }

    if let Some(p) = player_with_attack {
        let (transform, player, attack) = *p;
        let target_translation = &transform.translation
            + WEAPON_ATTACK_HORIZONTAL_OFFSET
                * (player.attack_direction * Vec2::new(1.0, 1.0)).extend(1.0);
        following_parts.iter_mut().for_each(|mut sprite| {
            sprite.color = color_with_transparency(1.0);
        });
        following.rotation = Quat::from_rotation_z(Vec2::Y.angle_to(player.attack_direction));
        following
            .translation
            .smooth_nudge(&target_translation, 5.0, delta_time);
    }
}
