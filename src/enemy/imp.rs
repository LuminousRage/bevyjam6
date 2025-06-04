use avian2d::{math::*, prelude::*};
use bevy::{
    math::ops::{abs, exp, sqrt},
    prelude::*,
};

use crate::{
    asset_tracking::LoadResource,
    enemy::configs::*,
    health::Health,
    physics::{
        configs::GRAVITY_ACCELERATION,
        creature::{CreaturePhysicsBundle, Grounded},
    },
    player::movement::CharacterController,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ImpAssets>();
    app.load_resource::<ImpAssets>()
        .add_systems(Update, enemy_decision_making);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ImpAssets {
    #[dependency]
    imp: Handle<Image>,
}

impl FromWorld for ImpAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            imp: assets.load("images/imp.png"),
        }
    }
}

pub fn imp(imp_assets: &ImpAssets, translation: Vec3) -> impl Bundle {
    (
        Name::new("Imp"),
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)).with_translation(translation),
        Sprite {
            image: imp_assets.imp.clone(),
            ..default()
        },
        ImpControllerBundle::new(Collider::circle(30.0)),
        Health::new(CHARACTER_HEALTH),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.5),
    )
}

#[derive(Component)]
pub struct ImpController {
    jump_attack_cooldown: f32,
    expected_time_until_jump_hits: f32,
    melee_attack_cooldown: f32,
}
impl ImpController {
    fn new() -> Self {
        Self {
            jump_attack_cooldown: 0.0,
            expected_time_until_jump_hits: 0.0,
            melee_attack_cooldown: 0.0,
        }
    }
}

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct ImpControllerBundle {
    imp_controller: ImpController,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    physics: CreaturePhysicsBundle,
}

impl ImpControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            imp_controller: ImpController::new(),
            body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(10.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            physics: CreaturePhysicsBundle::new(MOVEMENT_DAMPING, MAX_SLOPE_ANGLE),
        }
    }
}

fn enemy_decision_making(
    mut commands: Commands,
    time: Res<Time>,
    target: Query<&Transform, With<CharacterController>>,
    mut imps: Query<(
        Entity,
        &mut ImpController,
        &Transform,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    for (entity, mut imp, pos, mut velocity, is_grounded) in imps {
        let delta_time = time.delta_secs_f64().adjust_precision();
        imp.jump_attack_cooldown -= delta_time;
        imp.melee_attack_cooldown -= delta_time;
        imp.expected_time_until_jump_hits -= delta_time;
        let target_coords = target.single().unwrap().translation;
        let target_length = target_coords.x - pos.translation.x;
        let target_height = target_coords.y - pos.translation.y;

        //good time for a jump attack?
        if is_grounded
            && imp.jump_attack_cooldown <= 0.0
            && (abs(target_height) >= USE_JUMP_ATTACK_HEIGHT_DIFF_THRESHOLD
                || abs(target_length) >= USE_JUMP_ATTACK_MIN_LENGTH_THRESHOLD)
            && target_height <= 0.5 * JUMP_IMPULSE / GRAVITY_ACCELERATION
        {
            let time_til_target = (JUMP_IMPULSE
                + sqrt(JUMP_IMPULSE.powf(2.0) - 2.0 * GRAVITY_ACCELERATION * target_height))
                / GRAVITY_ACCELERATION;
            let x_velocity_to_reach_target = target_length
                / (1.0 / MOVEMENT_DAMPING
                    - 1.0 / MOVEMENT_DAMPING * exp(-time_til_target * MOVEMENT_DAMPING));
            //ATTACK!!!
            if x_velocity_to_reach_target <= MAX_X_VELOCITY {
                velocity.y += JUMP_IMPULSE;
                velocity.x = x_velocity_to_reach_target;
                imp.jump_attack_cooldown = JUMP_ATTACK_COOLDOWN;
                imp.expected_time_until_jump_hits = time_til_target;
                continue;
            }
        }

        // good time to melee attack?
        if is_grounded
            && abs(target_height) <= USE_MELEE_MAX_HEIGHT_DIFF
            && abs(target_length) <= USE_MELEE_MAX_LENGTH_DIFF
        {
            imp.melee_attack_cooldown = MELEE_ATTACK_COOLDOWN;
            continue;
        }

        // just run at them lmao
        if imp.expected_time_until_jump_hits < 0.0
            && (abs(target_height) > USE_MELEE_MAX_HEIGHT_DIFF
                || abs(target_length) > STOP_RUNNING_DISTANCE)
        {
            velocity.x += target_length.signum() * MOVEMENT_ACCELERATION * delta_time;
            continue;
        }

        //in range so stop
        if imp.expected_time_until_jump_hits < 0.0
            && (abs(target_height) <= USE_MELEE_MAX_HEIGHT_DIFF
                || abs(target_length) <= STOP_RUNNING_DISTANCE)
        {
            velocity.x +=
                -velocity.x.signum() * (MOVEMENT_ACCELERATION * delta_time).max(abs(velocity.x));
            continue;
        }
    }
}
