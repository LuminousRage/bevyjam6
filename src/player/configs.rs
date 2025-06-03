use avian2d::math::Scalar;

pub const MOVEMENT_ACCELERATION: f32 = 1250.0;
pub const DASH_SPEED_MODIFIER: f32 = 50.0;
pub const JUMP_IMPULSE: f32 = 700.0;
pub const MOVEMENT_DAMPING: f32 = 0.92;
pub const MAX_SLOPE_ANGLE: f32 = (30.0 as Scalar).to_radians();
