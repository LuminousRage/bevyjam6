use avian2d::math::Scalar;

pub const MOVEMENT_ACCELERATION: f32 = 1500.0;
pub const DASH_SPEED_MODIFIER: f32 = 50.0;
pub const JUMP_IMPULSE: f32 = 1100.0;
pub const MOVEMENT_DAMPING: f32 = 0.92;
pub const MAX_SLOPE_ANGLE: f32 = (30.0 as Scalar).to_radians();
pub const CHARACTER_GRAVITY_SCALE: f32 = 1.5;
pub const DASH_DURATION_SECONDS: f32 = 0.2;
