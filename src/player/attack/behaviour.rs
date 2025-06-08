use std::time::Duration;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<InputAttackEvent>();
    app.add_event::<DoAttackEvent>();
    app.add_event::<AttackDirection>();
}

const INITIAL_ATTACK_COOLDOWN_SECONDS: f32 = 2.;
const MINIMUM_ATTACK_COOLDOWN_SECONDS: f32 = 0.05;
const ATTACK_PERIOD_SECONDS: f32 = 2.;
const GRACE_PERIOD_SECONDS: f32 = 2.0;
const INITIAL_EXTEND_SCALE: f32 = 8.0;
const MINIMUM_EXTEND_SCALE: f32 = 1.0;

const SCALE_INCREASE_FACTOR: f32 = 1.2;
const SCALE_DECREASE_FACTOR: f32 = 0.8;
const COOLDOWN_INCREASE_FACTOR: f32 = 1.2;
const COOLDOWN_DECREASE_FACTOR: f32 = 0.8;

const WEAPON_ATTACK_HORIZONTAL_OFFSET: Vec3 = Vec3::new(-60.0, -47.0, -1.0);
const WEAPON_ATTACK_VERTICAL_OFFSET: Vec3 = Vec3::new(30.0, 10.0, -1.0);

#[derive(Event)]
pub struct InputAttackEvent;

#[derive(Event)]
pub struct DoAttackEvent;

#[derive(Event)]
pub struct AttackDirection(pub Vec2);

#[derive(Component)]
pub struct Attack {
    /// Current phase of the attack
    pub phase: AttackPhase,
    /// Determines the timer length for reacting phase
    pub attack_delay_seconds: f32,
    /// Weapon size multiplier
    pub extend_scale: f32,
    pub position: AttackPosition,
}

#[derive(Default)]
pub enum AttackPosition {
    #[default]
    Up,
    Down,
}

impl AttackPosition {
    pub fn get_next(&self) -> Self {
        match self {
            AttackPosition::Up => AttackPosition::Down,
            AttackPosition::Down => AttackPosition::Up,
        }
    }

    // todo: this code sucks, maybe fix it later
    pub fn get_translate(&self, attack_direction: Vec2) -> Vec3 {
        if attack_direction.x == 0.0 {
            let translation_offset = if let AttackPosition::Down = self {
                Vec3::new(-70.0, 0.0, 0.0)
            } else {
                Vec3::ZERO
            };

            (WEAPON_ATTACK_VERTICAL_OFFSET + translation_offset)
                * Vec3::new(1.0, attack_direction.y, 1.0)
        } else {
            let translation_offset = if let AttackPosition::Down = self {
                Vec3::new(0.0, 110.0, 0.0)
            } else {
                Vec3::ZERO
            };

            (WEAPON_ATTACK_HORIZONTAL_OFFSET + translation_offset)
                * Vec3::new(attack_direction.x, 1.0, 1.0)
        }
    }

    pub fn get_scale(&self, attack_direction: Vec2) -> Vec3 {
        let is_vertical = attack_direction.x == 0.0;
        let is_positive = if is_vertical {
            attack_direction.y > 0.0
        } else {
            attack_direction.x > 0.0
        };

        match (is_vertical, is_positive, self) {
            (true, false, AttackPosition::Up)
            | (true, true, AttackPosition::Down)
            | (false, true, AttackPosition::Down)
            | (false, false, AttackPosition::Up) => Vec3::new(1.0, 1.0, 1.0),
            _ => Vec3::new(-1.0, 1.0, 1.0),
        }
    }
}

pub enum AttackPhase {
    /// Weapon is chain reacting, timer is how long from button press to attack
    Reacting(Timer),
    /// Attack animation time, vec holds translation of the character at attack time
    Attacking { pos: Vec3, direction: Vec2 },
    /// Weapon is ready to attack, timer is how long until weapon starting cooling down
    Ready(Timer),
    /// Weapon is cooling down, attacking during this period will increase the cooldown
    Cooling(Timer),
}

impl Default for AttackPhase {
    fn default() -> Self {
        AttackPhase::Reacting(Timer::from_seconds(
            INITIAL_ATTACK_COOLDOWN_SECONDS,
            TimerMode::Once,
        ))
    }
}

impl AttackPhase {
    pub fn tick(&mut self, time: Duration) {
        match self {
            AttackPhase::Reacting(timer) => {
                timer.tick(time);
            }
            AttackPhase::Attacking {
                pos: _,
                direction: _,
            } => {}
            AttackPhase::Ready(timer) => {
                timer.tick(time);
            }
            AttackPhase::Cooling(timer) => {
                timer.tick(time);
            }
        };
    }
    pub fn new_ready_timer() -> AttackPhase {
        AttackPhase::Ready(Timer::from_seconds(ATTACK_PERIOD_SECONDS, TimerMode::Once))
    }

    pub fn new_cooling_timer() -> AttackPhase {
        AttackPhase::Cooling(Timer::from_seconds(GRACE_PERIOD_SECONDS, TimerMode::Once))
    }
}

impl Default for Attack {
    fn default() -> Self {
        Attack::new(INITIAL_ATTACK_COOLDOWN_SECONDS, INITIAL_EXTEND_SCALE)
    }
}
impl Attack {
    pub fn new(attack_delay: f32, extend_scale: f32) -> Self {
        Self {
            phase: AttackPhase::default(),
            attack_delay_seconds: attack_delay,
            extend_scale,
            position: AttackPosition::default(),
        }
    }

    pub fn update_fury(&mut self, increase_fury: bool) {
        // that u128 duration cast to f64 should be fine
        // because it should never be bigger than INITIAL_ATTACK_COOLDOWN_MILLISECONDS
        if increase_fury {
            let decreased_cooldown = self.attack_delay_seconds * COOLDOWN_DECREASE_FACTOR;
            self.attack_delay_seconds = decreased_cooldown.max(MINIMUM_ATTACK_COOLDOWN_SECONDS);
            self.extend_scale =
                (self.extend_scale * SCALE_DECREASE_FACTOR).max(MINIMUM_EXTEND_SCALE);
        } else {
            let increased_cooldown = self.attack_delay_seconds * COOLDOWN_INCREASE_FACTOR;
            self.attack_delay_seconds = increased_cooldown.min(INITIAL_ATTACK_COOLDOWN_SECONDS);
            self.extend_scale =
                (self.extend_scale * SCALE_INCREASE_FACTOR).min(INITIAL_EXTEND_SCALE);
        };
    }

    pub fn new_reaction_timer(&self) -> AttackPhase {
        AttackPhase::Reacting(Timer::from_seconds(
            self.attack_delay_seconds,
            TimerMode::Once,
        ))
    }
}
