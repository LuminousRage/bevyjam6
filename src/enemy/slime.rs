use avian2d::{math::*, prelude::*};
use bevy::{
    math::ops::{abs, exp, sqrt},
    prelude::*,
};

use crate::{
    asset_tracking::LoadResource,
    collision_layers::{GameLayer, enemy_hit_boxes, enemy_hurt_boxes},
    enemy::configs::*,
    health::{DeathEvent, Health, hitbox_prefab, hurtbox_prefab},
    physics::{
        configs::GRAVITY_ACCELERATION,
        creature::{CreaturePhysicsBundle, Grounded, MovementDampingFactor},
    },
    player::character::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SlimeAssets>();
    app.load_resource::<SlimeAssets>()
        .add_systems(Update, enemy_decision_making)
        .add_systems(Last, kill_slimes);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct SlimeAssets {
    #[dependency]
    slime1: Handle<Image>,
    slime2: Handle<Image>,
}

impl FromWorld for SlimeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            slime1: assets.load("images/SLIME_V1-01.png"),
            slime2: assets.load("images/SLIME_V1-02.png"),
        }
    }
}

pub fn slime(slime_assets: &SlimeAssets, translation: Vec3, is_red: bool) -> impl Bundle {
    let scale = Vec2::splat(0.5);
    (
        Name::new("Slime"),
        Transform::from_scale(scale.extend(1.0)).with_translation(translation),
        Sprite {
            image: if is_red {
                slime_assets.slime1.clone()
            } else {
                slime_assets.slime2.clone()
            },
            custom_size: Some(Vec2::new(4500.0 / 30., 3127.0 / 30.)),
            ..default()
        },
        SlimeControllerBundle::new(
            Collider::circle(55.0),
            scale,
            if is_red {
                RED_MAX_X_VELOCITY
            } else {
                BLACK_MAX_X_VELOCITY
            },
            if is_red {
                RED_JUMP_ATTACK_COOLDOWN
            } else {
                BLACK_JUMP_ATTACK_COOLDOWN
            },
        ),
        Health::new(if is_red { RED_HEALTH } else { BLACK_HEALTH }),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.0),
        children![
            hurtbox_prefab(
                Collider::circle(60.0),
                enemy_hurt_boxes(),
                0.5,
                Transform::default()
            ),
            hitbox_prefab(
                Collider::circle(60.0),
                enemy_hit_boxes(),
                0.5,
                if is_red { 15.0 } else { 8.0 },
                Transform::default(),
            )
        ],
    )
}

#[derive(Component)]
pub struct SlimeController {
    max_x_velocity: f32,
    jump_attack_full_cooldown: f32,
    jump_attack_cooldown: f32,
    expected_time_until_jump_hits: f32,
}
impl SlimeController {
    fn new(max_x_velocity: f32, jump_attack_full_cooldown: f32) -> Self {
        Self {
            max_x_velocity,
            jump_attack_full_cooldown,
            jump_attack_cooldown: 0.0,
            expected_time_until_jump_hits: 0.0,
        }
    }
}

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct SlimeControllerBundle {
    slime_controller: SlimeController,
    physics: CreaturePhysicsBundle,
}

impl SlimeControllerBundle {
    pub fn new(
        collider: Collider,
        scale: Vector,
        max_x_velocity: f32,
        jump_attack_full_cooldown: f32,
    ) -> Self {
        Self {
            slime_controller: SlimeController::new(max_x_velocity, jump_attack_full_cooldown),
            physics: CreaturePhysicsBundle::new(collider, scale, MOVEMENT_DAMPING, MAX_SLOPE_ANGLE),
        }
    }
}

fn enemy_decision_making(
    mut commands: Commands,
    time: Res<Time>,
    target: Single<&Transform, With<Player>>,
    mut slimes: Query<(
        Entity,
        &mut SlimeController,
        &Transform,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    for (entity, mut slime, pos, mut velocity, is_grounded) in slimes {
        let delta_time = time.delta_secs_f64().adjust_precision();
        slime.jump_attack_cooldown -= delta_time;
        slime.expected_time_until_jump_hits -= delta_time;
        let target_coords = target.translation;
        let target_length = target_coords.x - pos.translation.x;
        let target_height = (target_coords.y - pos.translation.y)
            .min(0.5 * JUMP_IMPULSE.powf(2.0) / GRAVITY_ACCELERATION);

        //good time for a jump attack?
        if is_grounded && slime.jump_attack_cooldown <= 0.0 {
            let time_til_target = (JUMP_IMPULSE
                + sqrt(JUMP_IMPULSE.powf(2.0) - 2.0 * GRAVITY_ACCELERATION * target_height))
                / GRAVITY_ACCELERATION;
            //just assume no dampening
            let x_velocity_to_reach_target =
                (abs(target_length) / time_til_target).min(slime.max_x_velocity);
            //ATTACK!!!
            velocity.y += JUMP_IMPULSE;
            velocity.x = target_length.signum() * x_velocity_to_reach_target;
            slime.jump_attack_cooldown = slime.jump_attack_full_cooldown + time_til_target / 2.0;
            continue;
        }
        if is_grounded && slime.jump_attack_cooldown < slime.jump_attack_full_cooldown {
            velocity.x = 0.0;
        }
    }
}

fn kill_slimes(
    mut commands: Commands,
    mut death_reader: EventReader<DeathEvent>,
    mut slimes: Query<(Entity, &mut SlimeController)>,
) {
    for DeathEvent(entity) in death_reader.read() {
        if slimes.contains(*entity) {
            commands.entity(*entity).despawn();
        }
    }
}
