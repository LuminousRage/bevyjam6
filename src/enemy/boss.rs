use crate::enemy::eye::{EyeAssets, the_eye};
use avian2d::{math::*, prelude::*};
use bevy::{
    math::ops::{exp, sqrt},
    prelude::*,
};
use rand::Rng;

use crate::{
    enemy::configs::*,
    health::Health,
    physics::creature::{CreaturePhysicsBundle, Flying, Grounded},
    player::movement::CharacterController,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, enemy_decision_making);
}

// #[derive(Resource, Asset, Clone, Reflect)]
// #[reflect(Resource)]
// pub struct BossAssets {
//     #[dependency]
//     boss: Handle<Image>,
// }

// impl FromWorld for BossAssets {
//     fn from_world(world: &mut World) -> Self {
//         let assets = world.resource::<AssetServer>();
//         Self {
//             boss: assets.load("images/boss.png"),
//         }
//     }
// }

pub fn boss(
    eye_assets: &EyeAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    translation: Vec3,
) -> impl Bundle {
    let scale = Vec2::splat(1.0);
    (
        Name::new("Boss"),
        the_eye(&eye_assets, texture_atlas_layouts, scale, translation),
        BossController::new(),
        // Flying,
        // CreaturePhysicsBundle::new(
        //     Collider::circle(300.),
        //     scale,
        //     MOVEMENT_DAMPING,
        //     MAX_SLOPE_ANGLE,
        // ),
        Health::new(BOSS_HEALTH),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(0.),
    )
}

#[derive(Component)]
pub struct BossController {
    unchained: bool,
    time_until_next_attack: f32,
    time_since_last_reposition_ended: f32,
    repositioning_to_left: bool,
}
impl BossController {
    fn new() -> Self {
        Self {
            unchained: false,
            time_until_next_attack: 0.0,
            time_since_last_reposition_ended: 0.0,
            repositioning_to_left: false,
        }
    }
}

fn enemy_decision_making(
    mut commands: Commands,
    time: Res<Time>,
    target: Query<&Transform, With<CharacterController>>,
    mut bosses: Query<(Entity, &mut BossController, &mut Transform), Without<CharacterController>>,
) {
    for (entity, mut boss, mut pos) in bosses {
        let delta_time = time.delta_secs_f64().adjust_precision();
        //reapersitioning
        if boss.time_since_last_reposition_ended < 0. {
            boss.time_since_last_reposition_ended += delta_time;
            let current_t: f32 = if boss.repositioning_to_left {
                TIME_TO_REPOSITION + boss.time_since_last_reposition_ended.min(0.)
            } else {
                -boss.time_since_last_reposition_ended.min(0.)
            };
            let a: f32 = sqrt(2.) * (POSITION_2_X - POSITION_1.x);
            let b: f32 = (2. / (2. + sqrt(2.))) * (MAX_REPOSITIONING_Y - POSITION_1.y);
            let x_trans: f32 = (POSITION_2_X + POSITION_1.x) / 2.;
            let y_trans: f32 = MAX_REPOSITIONING_Y - b;
            let lerp = std::f32::consts::PI / 4.
                * (-1. * (1. - current_t / TIME_TO_REPOSITION)
                    + 5. * current_t / TIME_TO_REPOSITION);
            pos.translation.x = a * lerp.cos() + x_trans;
            pos.translation.y = b * lerp.sin() + y_trans;
            continue;
        }

        boss.time_until_next_attack -= delta_time;
        boss.time_since_last_reposition_ended += delta_time;

        //attacks
        let roll: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let target_coords = target.single().unwrap().translation;
        let relative_coords = target_coords - pos.translation;
        //good time for a reposition attack?
        if boss.time_until_next_attack <= 0.0
            && relative_coords.length_squared() <= 600_f32.powf(2.)
            && roll.powf(1. / delta_time)
                < 1.0 / (1. + exp(-0.7 * (boss.time_since_last_reposition_ended - 15.0)))
        {
            boss.time_until_next_attack = BOSS_TIME_BETWEEN_ATTACKS;
            boss.time_since_last_reposition_ended = -TIME_TO_REPOSITION;
            boss.repositioning_to_left = !boss.repositioning_to_left;
        }

        //good time for a beam attack?
        if boss.time_until_next_attack <= 0.0
            && relative_coords.length_squared() <= 800_f32.powf(2.)
        {
            // commands.spawn(lazer())
        }
        //always a good time for a sky beam attack
        if boss.time_until_next_attack <= 0.0 {}
    }
}
