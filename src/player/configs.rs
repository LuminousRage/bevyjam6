use avian2d::math::Scalar;

pub const MOVEMENT_ACCELERATION: f32 = 5000.0;
pub const DASH_SPEED_MODIFIER: f32 = 12.5;
pub const JUMP_IMPULSE: f32 = 1100.0;
pub const MOVEMENT_DAMPING: f32 = 6.0;
pub const MAX_SLOPE_ANGLE: f32 = (30.0 as Scalar).to_radians();
pub const CHARACTER_GRAVITY_SCALE: f32 = 1.5;
pub const DASH_DURATION_SECONDS: f32 = 0.2;
pub const JUMP_DURATION_SECONDS: f32 = 0.5;

pub const CHARACTER_HEALTH: f32 = 100.0;
