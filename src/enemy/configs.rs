use avian2d::math::Scalar;

pub const JUMP_IMPULSE: f32 = 1100.0;
pub const MOVEMENT_DAMPING: f32 = 8.0;
pub const MAX_SLOPE_ANGLE: f32 = (30.0 as Scalar).to_radians();
// pub const CHARACTER_GRAVITY_SCALE: f32 = 1.5;
pub const JUMP_DURATION_SECONDS: f32 = 0.5;

pub const CHARACTER_HEALTH: f32 = 30.0;
pub const JUMP_ATTACK_COOLDOWN: f32 = 10.0;
pub const MELEE_ATTACK_COOLDOWN: f32 = 1.0;
pub const MAX_X_VELOCITY: f32 = 30.0;
pub const USE_JUMP_ATTACK_HEIGHT_DIFF_THRESHOLD: f32 = 30.0;
pub const USE_JUMP_ATTACK_MIN_LENGTH_THRESHOLD: f32 = 30.0;
pub const USE_MELEE_MAX_HEIGHT_DIFF: f32 = 100.0;
pub const USE_MELEE_MAX_LENGTH_DIFF: f32 = 150.0;
pub const STOP_RUNNING_DISTANCE: f32 = USE_MELEE_MAX_LENGTH_DIFF;
pub const MOVEMENT_ACCELERATION: f32 = 3000.0;

//TODO:need some notion of attack delays, after landing?
