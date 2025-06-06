use avian2d::{math::*, prelude::*};
use bevy::{
    math::ops::{abs, exp, sqrt},
    prelude::*,
};

use crate::{
    asset_tracking::LoadResource,
    collision_layers::{GameLayer, enemy_hit_boxes, enemy_hurt_boxes},
    enemy::configs::*,
    health::{Health, hitbox_prefab, hurtbox_prefab},
    physics::{
        configs::GRAVITY_ACCELERATION,
        creature::{CreaturePhysicsBundle, Grounded, MovementDampingFactor},
    },
    player::movement::CharacterController,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SlimeAssets>();
    app.load_resource::<SlimeAssets>()
        .add_systems(Update, enemy_decision_making);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct SlimeAssets {
    #[dependency]
    slime: Handle<Image>,
}

impl FromWorld for SlimeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            slime: assets.load("images/slime.png"),
        }
    }
}

pub fn slime(slime_assets: &SlimeAssets, translation: Vec3) -> impl Bundle {
    let scale = Vec2::splat(0.5);
    (
        Name::new("Slime"),
        Transform::from_scale(scale.extend(1.0)).with_translation(translation),
        Sprite {
            image: slime_assets.slime.clone(),
            ..default()
        },
        SlimeControllerBundle::new(Collider::circle(30.0), scale),
        Health::new(CHARACTER_HEALTH),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.0),
        children![
            hurtbox_prefab(
                Collider::circle(40.0),
                enemy_hurt_boxes(),
                0.5,
                Transform::default()
            ),
            hitbox_prefab(Collider::circle(40.0), enemy_hit_boxes(), 0.5, 10.0)
        ],
    )
}

#[derive(Component)]
pub struct SlimeController {
    jump_attack_cooldown: f32,
    expected_time_until_jump_hits: f32,
}
impl SlimeController {
    fn new() -> Self {
        Self {
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
    pub fn new(collider: Collider, scale: Vector) -> Self {
        Self {
            slime_controller: SlimeController::new(),
            physics: CreaturePhysicsBundle::new(collider, scale, MOVEMENT_DAMPING, MAX_SLOPE_ANGLE),
        }
    }
}

fn enemy_decision_making(
    mut commands: Commands,
    time: Res<Time>,
    target: Query<&Transform, With<CharacterController>>,
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
        let target_coords = target.single().unwrap().translation;
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
                (abs(target_length) / time_til_target).min(MAX_X_VELOCITY);
            //ATTACK!!!
            velocity.y += JUMP_IMPULSE;
            velocity.x = target_length.signum() * x_velocity_to_reach_target;
            slime.jump_attack_cooldown = JUMP_ATTACK_COOLDOWN + time_til_target;
            continue;
        }
        if is_grounded && slime.jump_attack_cooldown < JUMP_ATTACK_COOLDOWN {
            velocity.x = 0.0;
        }
    }
}
