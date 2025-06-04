use avian2d::math::Scalar;
use bevy::input::keyboard::KeyCode;

pub const MOVEMENT_SPEED: f32 = 500.0;
pub const DASH_SPEED_MODIFIER: f32 = 60.;
pub const JUMP_IMPULSE: f32 = 800.0;
pub const MOVEMENT_DAMPING: f32 = 6.0;
pub const MAX_SLOPE_ANGLE: f32 = (30.0 as Scalar).to_radians();
pub const CHARACTER_GRAVITY_SCALE: f32 = 1.5;
pub const DASH_DURATION_MILLISECONDS: u64 = 200;
pub const JUMP_DURATION_MILLISECONDS: u64 = 400;

pub const CHARACTER_HEALTH: f32 = 100.0;

pub const KEYBOARD_LEFT: KeyCode = KeyCode::ArrowLeft;
pub const KEYBOARD_RIGHT: KeyCode = KeyCode::ArrowRight;
pub const KEYBOARD_DOWN: KeyCode = KeyCode::ArrowDown;
pub const KEYBOARD_UP: KeyCode = KeyCode::ArrowUp;
pub const KEYBOARD_JUMP: KeyCode = KeyCode::KeyZ;
pub const KEYBOARD_ATTACK: KeyCode = KeyCode::KeyX;
pub const KEYBOARD_DASH: KeyCode = KeyCode::KeyC;
