use avian2d::math::Scalar;

pub const MOVEMENT_ACCELERATION: f32 = 2500.0;
pub const MOVEMENT_SPEED: f32 = 500.0;
pub const DASH_SPEED_MODIFIER: f32 = 55.5;
pub const JUMP_IMPULSE: f32 = 1000.0;
pub const MOVEMENT_DAMPING: f32 = 6.0;
pub const MAX_SLOPE_ANGLE: f32 = (30.0 as Scalar).to_radians();
pub const CHARACTER_GRAVITY_SCALE: f32 = 1.5;
pub const DASH_DURATION_MILLISECONDS: u64 = 200;
pub const JUMP_DURATION_MILLISECONDS: u64 = 600;

pub const CHARACTER_HEALTH: f32 = 100.0;
