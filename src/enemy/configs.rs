use avian2d::math::Scalar;
use bevy::math::Vec2;

pub const JUMP_IMPULSE: f32 = 650.0;
pub const MOVEMENT_DAMPING: f32 = 0.0;
pub const MAX_SLOPE_ANGLE: f32 = (30.0 as Scalar).to_radians();
pub const BOSS_HEALTH: f32 = 2000.0;
pub const BOSS_TIME_BETWEEN_ATTACKS: f32 = 3.0;
pub const TIME_TO_REPOSITION: f32 = 3.5;
pub const POSITION_1: Vec2 = Vec2::new(-200., 100.);
pub const POSITION_2_X: f32 = -POSITION_1.x;
pub const MAX_REPOSITIONING_Y: f32 = 400.0;
pub const SKY_LAZER_DURATION: f32 = 2.;
pub const SKY_LAZER_SPAWN_FREQUENCY: f32 = 0.3;
pub const SKY_ATTACK_DURATION: f32 = 5.0;
pub const SKY_ATTACK_START_TIME: f32 = 0.25;
pub const BEAM_LAZER_DURATION: f32 = 1.65;
// pub const BEAM_LAZER_WIDTH: f32 = 2.;
pub const BEAM_ATTACK_DURATION: f32 = BEAM_LAZER_DURATION + 0.35;
//slime stuff
pub const RED_HEALTH: f32 = 25.0;
pub const RED_JUMP_ATTACK_COOLDOWN: f32 = 2.0;
pub const RED_MAX_X_VELOCITY: f32 = 325.0;
pub const BLACK_HEALTH: f32 = 40.0;
pub const BLACK_JUMP_ATTACK_COOLDOWN: f32 = 3.5;
pub const BLACK_MAX_X_VELOCITY: f32 = 250.0;
pub const MOVEMENT_ACCELERATION: f32 = 3000.0;

//imp stuff
pub const USE_JUMP_ATTACK_HEIGHT_DIFF_THRESHOLD: f32 = 30.0;
pub const USE_JUMP_ATTACK_MIN_LENGTH_THRESHOLD: f32 = 30.0;
pub const MELEE_ATTACK_COOLDOWN: f32 = 1.0;
pub const USE_MELEE_MAX_HEIGHT_DIFF: f32 = 100.0;
pub const USE_MELEE_MAX_LENGTH_DIFF: f32 = 150.0;
pub const STOP_RUNNING_DISTANCE: f32 = USE_MELEE_MAX_LENGTH_DIFF;
