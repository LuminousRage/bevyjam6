use crate::enemy::eye::{EyeAssets, the_eye};
use avian2d::{math::*, prelude::*};
use bevy::{
    math::ops::{exp, sqrt},
    prelude::*,
};
use rand::Rng;

use crate::{enemy::configs::*, health::Health, player::movement::CharacterController};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, enemy_decision_making);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LazerAssets {
    #[dependency]
    boss: Handle<Image>,
}

impl FromWorld for LazerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            boss: assets.load("images/LASER_BEAM.png"),
        }
    }
}

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
        Health::new(BOSS_HEALTH),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(0.),
    )
}
pub fn lazer(lazer_assets: &LazerAssets, translation: Vec3, direction: Vec3) -> impl Bundle {
    let scale = Vec2::splat(1.0);
    (
        Name::new("Lazer"),
        BossController::new(),
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
    pub sky_lazer_remaining_duration: f32,
    pub beam_lazer_remaining_duration: f32,
    repositioning_to_left: bool,
}
impl BossController {
    fn new() -> Self {
        Self {
            unchained: false,
            time_until_next_attack: 0.0,
            time_since_last_reposition_ended: 0.0,
            sky_lazer_remaining_duration: 0.0,
            beam_lazer_remaining_duration: 0.0,
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
        boss.time_until_next_attack -= delta_time;
        boss.time_since_last_reposition_ended += delta_time;
        boss.sky_lazer_remaining_duration -= delta_time;
        boss.beam_lazer_remaining_duration -= delta_time;
        if boss.time_since_last_reposition_ended - delta_time < 0. {
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

        //attacks
        let roll: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let target_coords = target.single().unwrap().translation;
        let relative_coords = target_coords - pos.translation;
        //good time for a reposition attack?
        if boss.time_until_next_attack <= 0.0
            && relative_coords.length_squared() <= 600_f32.powf(2.)
            && roll.powf(1.5 / delta_time)
                > 1. - 1.0 / (1. + exp(-0.7 * (boss.time_since_last_reposition_ended - 15.0)))
        {
            // dbg!(roll);
            // dbg!(boss.time_since_last_reposition_ended);
            // dbg!(roll.powf(1.5 / delta_time));
            // dbg!(1. - 1.0 / (1. + exp(-0.7 * (boss.time_since_last_reposition_ended - 15.0))));
            boss.time_until_next_attack = BOSS_TIME_BETWEEN_ATTACKS + TIME_TO_REPOSITION;
            boss.time_since_last_reposition_ended = -TIME_TO_REPOSITION;
            boss.repositioning_to_left = !boss.repositioning_to_left;
            continue;
        }

        //good time for a beam attack?
        if boss.time_until_next_attack <= 0.0
            && relative_coords.length_squared() <= 500_f32.powf(2.)
        {
            boss.time_until_next_attack = BOSS_TIME_BETWEEN_ATTACKS + BEAM_ATTACK_DURATION;
            boss.beam_lazer_remaining_duration = BEAM_ATTACK_DURATION;
            // commands.spawn(lazer());
            continue;
        }
        //always a good time for a sky beam attack
        if boss.time_until_next_attack <= 0.0 {
            boss.sky_lazer_remaining_duration = SKY_ATTACK_DURATION;
            boss.time_until_next_attack = BOSS_TIME_BETWEEN_ATTACKS + SKY_ATTACK_DURATION;
            // commands.spawn(lazer());
            // commands.spawn(lazer());
            continue;
        }

        // pub const SKY_LAZER_DURATION: f32 = 1.;
        // pub const SKY_LAZER_SPAWN_FREQUENCY: f32 = 0.5;
        // pub const SKY_LAZER_ATTACK_DURATION: f32 = 5.0;
        // pub const BEAM_LAZER_DURATION: f32 = 1.65;
        // pub const BEAM_LAZER_WIDTH: f32 = 2.;
        // pub const BEAM_ATTACK_DURATION: f32 = BEAM_LAZER_DURATION + 0.35;
    }
}
