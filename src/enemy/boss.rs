use crate::{
    PausableSystems,
    asset_tracking::LoadResource,
    audio::sound_effect,
    collision_layers::enemy_hurt_boxes,
    enemy::eye::{EyeAssets, Pupil, RayWhite, the_eye},
    health::hurtbox_prefab,
    player::character::Player,
};
use avian2d::{math::*, prelude::*};
use bevy::{
    math::ops::{exp, sqrt},
    prelude::*,
    sprite::Anchor,
};
use rand::{Rng, seq::SliceRandom};
use statrs::distribution::{ContinuousCDF, Normal};

use crate::enemy::configs::*;
use crate::health::Health;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LazerAssets>();
    app.load_resource::<LazerAssets>();
    app.add_systems(
        Update,
        (enemy_decision_making, tick_lazers).in_set(PausableSystems),
    );
}

const BEAM_LASER_SCALE: Vec3 = Vec3::new(5.0, 1.0, 1.);
const RAINING_LASER_SCALE: Vec3 = Vec3::ONE;

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
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LazerAssets {
    #[dependency]
    img: Handle<Image>,
    #[dependency]
    glint: Handle<Image>,
    #[dependency]
    laser_long: Handle<AudioSource>,
    #[dependency]
    laser_short: Vec<Handle<AudioSource>>,
}

impl FromWorld for LazerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            img: assets.load("images/LASER_BEAM.png"),
            glint: assets.load("images/GLINT.png"),
            laser_long: assets.load("audio/sound_effects/player/laser_long.ogg"),
            laser_short: vec![
                assets.load("audio/sound_effects/player/laser_short_1.ogg"),
                assets.load("audio/sound_effects/player/laser_short_2.ogg"),
            ],
        }
    }
}
pub fn lazer(
    lazer_assets: &LazerAssets,
    translation: Vec3,
    direction: Vec3,
    time_remaining: f32,
    scale: Vec3,
    // this is just saying do we want sky or no sky
    with_glint: bool,
) -> impl Bundle {
    let size = Vec2::new(6035.0 / 10., 477.0 / 10.);
    let visibility = if with_glint {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    let sound_effect = if with_glint {
        sound_effect(lazer_assets.laser_long.clone())
    } else {
        let rng = &mut rand::thread_rng();
        let sound = lazer_assets.laser_short.choose(rng).unwrap().clone();
        sound_effect(sound)
    };

    let glint = (
        Name::new("Glint"),
        Sprite::from_image(lazer_assets.glint.clone()),
        Transform::default()
            .with_translation(-translation)
            .with_rotation(Quat::from_rotation_z(direction.angle_between(-Vec3::AXES[0])).inverse())
            .with_scale(-scale),
        visibility,
    );
    (
        Name::new("Lazer"),
        Sprite {
            image: lazer_assets.img.clone(),
            custom_size: Some(size),
            anchor: Anchor::Custom(Vec2::new(5820. / 6035. - 0.5, 0.0)),
            ..default()
        },
        Transform::default()
            .with_translation(translation)
            .with_rotation(Quat::from_rotation_z(
                direction.angle_between(-Vec3::AXES[0]),
            ))
            .with_scale(scale),
        children![
            hurtbox_prefab(
                Collider::rectangle(0.95 * size.x * scale.x, 0.2 * size.y),
                enemy_hurt_boxes(),
                0.05,
                Transform::from_translation(Vec3::new(
                    -(5820. / 6035. - 0.5) * 0.95 * size.x,
                    0.,
                    0.
                )),
            ),
            sound_effect,
        ],
        Lazer { time_remaining },
    )
}

#[derive(Component)]
pub struct Lazer {
    time_remaining: f32,
}

fn tick_lazers(mut commands: Commands, time: Res<Time>, lazers: Query<(Entity, &mut Lazer)>) {
    for (entity, mut lazer) in lazers {
        lazer.time_remaining -= time.delta_secs().adjust_precision();
        if lazer.time_remaining < 0.0 {
            commands.entity(entity).despawn();
        }
    }
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
            repositioning_to_left: true,
        }
    }
}

fn enemy_decision_making(
    mut commands: Commands,
    time: Res<Time>,
    target: Single<&Transform, With<Player>>,
    pupil: Single<&GlobalTransform, (With<Pupil>, Without<Player>, Without<RayWhite>)>,
    mut eye_white: Single<&mut Sprite, (With<RayWhite>, Without<Player>, Without<Pupil>)>,
    boss: Single<(Entity, &mut BossController, &mut Transform), (Without<Pupil>, Without<Player>)>,
    lazer_assets: Res<LazerAssets>,
    eye_assets: Res<EyeAssets>,
) {
    let (entity, mut boss, mut pos) = boss.into_inner();
    let delta_time = time.delta_secs_f64().adjust_precision();
    //reapersitioning
    boss.time_until_next_attack -= delta_time;
    boss.time_since_last_reposition_ended += delta_time;
    boss.sky_lazer_remaining_duration -= delta_time;
    boss.beam_lazer_remaining_duration -= delta_time;
    eye_white.image = eye_assets.white.clone();

    if boss.time_since_last_reposition_ended - delta_time < 0. {
        let current_t: f32 = if boss.repositioning_to_left {
            TIME_TO_REPOSITION + boss.time_since_last_reposition_ended.min(0.)
        } else {
            -boss.time_since_last_reposition_ended.min(0.)
        };
        let a: f32 = sqrt(2.) * (POSITION_2_X - POSITION_1.x) / 2.;
        let b: f32 = (MAX_REPOSITIONING_Y - POSITION_1.y) / (1. + 1. / sqrt(2.0));
        let x_trans: f32 = (POSITION_2_X + POSITION_1.x) / 2.;
        let y_trans: f32 = MAX_REPOSITIONING_Y - b;
        let lerp = std::f32::consts::PI / 4.
            * (-1. * (1. - current_t / TIME_TO_REPOSITION) + 5. * current_t / TIME_TO_REPOSITION);
        pos.translation.x = a * lerp.cos() + x_trans;
        pos.translation.y = b * lerp.sin() + y_trans;
        return;
    }

    //sky beamin
    if boss.sky_lazer_remaining_duration > 0. {
        eye_white.image = eye_assets.red.clone();

        //start lazer
        if boss.sky_lazer_remaining_duration + delta_time < SKY_ATTACK_START_TIME
            && boss.sky_lazer_remaining_duration >= SKY_ATTACK_START_TIME
        {
            commands.spawn(lazer(
                &lazer_assets,
                pupil.translation().with_z(7.),
                Vec3::new(0.0, 1.0, 0.0),
                boss.sky_lazer_remaining_duration,
                RAINING_LASER_SCALE,
                false,
            ));
        }
        //spawn lazers
        if (boss.sky_lazer_remaining_duration + delta_time) % SKY_LAZER_SPAWN_FREQUENCY
            < boss.sky_lazer_remaining_duration % SKY_LAZER_SPAWN_FREQUENCY
        {
            if let Ok(dist) = Normal::new(target.translation.x.into(), 200.0) {
                let roll: f64 = rand::thread_rng().gen_range(0.0..1.0);

                commands.spawn((
                    lazer(
                        &lazer_assets,
                        Vec3::new(dist.inverse_cdf(roll) as f32, 1200., 7.),
                        Vec3::new(0.0, -1.0, 0.0),
                        SKY_LAZER_DURATION,
                        RAINING_LASER_SCALE,
                        false,
                    ),
                    Collider::capsule(1., 1.),
                    CollisionLayers::new(0b00010, 0b00000),
                    RigidBody::Dynamic,
                    GravityScale(0.8),
                ));
            }
            return;
        }
    }

    //attacks
    let roll: f32 = rand::thread_rng().gen_range(0.0..1.0);
    let target_coords = target.translation;
    let relative_coords = target_coords - pos.translation;
    //good time for a reposition attack?
    if boss.time_until_next_attack <= 0.0
        && relative_coords.length_squared() <= 600_f32.powf(2.)
        && roll.powf(1.0 / delta_time)
            > 1. - 1.0 / (1. + exp(-0.7 * (boss.time_since_last_reposition_ended - 15.0)))
    {
        // dbg!(roll);
        // dbg!(boss.time_since_last_reposition_ended);
        // dbg!(roll.powf(1.5 / delta_time));
        // dbg!(1. - 1.0 / (1. + exp(-0.7 * (boss.time_since_last_reposition_ended - 15.0))));
        boss.time_until_next_attack = BOSS_TIME_BETWEEN_ATTACKS + TIME_TO_REPOSITION;
        boss.time_since_last_reposition_ended = -TIME_TO_REPOSITION;
        boss.repositioning_to_left = !boss.repositioning_to_left;
        return;
    }

    //good time for a beam attack?
    if boss.time_until_next_attack <= 0.0 && relative_coords.length_squared() <= 500_f32.powf(2.) {
        boss.time_until_next_attack = BOSS_TIME_BETWEEN_ATTACKS + BEAM_ATTACK_DURATION;
        boss.beam_lazer_remaining_duration = BEAM_ATTACK_DURATION;
        commands.spawn(lazer(
            &lazer_assets,
            pupil.translation().with_z(7.),
            target.translation - pupil.translation(),
            BEAM_LAZER_DURATION,
            BEAM_LASER_SCALE,
            true,
        ));
        // commands.spawn(lazer());
        return;
    }
    //always a good time for a sky beam attack
    if boss.time_until_next_attack <= 0.0 {
        boss.sky_lazer_remaining_duration = SKY_ATTACK_DURATION;
        boss.time_until_next_attack = BOSS_TIME_BETWEEN_ATTACKS + SKY_ATTACK_DURATION;
        return;
    }

    // pub const SKY_LAZER_DURATION: f32 = 1.;
    // pub const SKY_LAZER_SPAWN_FREQUENCY: f32 = 0.5;
    // pub const SKY_LAZER_ATTACK_DURATION: f32 = 5.0;
    // pub const BEAM_LAZER_DURATION: f32 = 1.65;
    // pub const BEAM_LAZER_WIDTH: f32 = 2.;
    // pub const BEAM_ATTACK_DURATION: f32 = BEAM_LAZER_DURATION + 0.35;
}
