use std::time::Duration;

use bevy::prelude::*;

use crate::physics::creature::Grounded;

use super::{
    character::Player,
    input::{gamepad_attack_input, keyboard_attack_input},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<InputAttackEvent>();
    app.add_event::<AttackDirection>();
    app.add_systems(
        Update,
        (
            (
                keyboard_attack_input,
                gamepad_attack_input,
                player_attack_direction,
            ),
            (attack_handler, do_attack).chain(),
        )
            .chain(),
    );
}

const INITIAL_ATTACK_COOLDOWN_SECONDS: f32 = 2.;
const MINIMUM_ATTACK_COOLDOWN_SECONDS: f32 = 1.;
const ATTACK_PERIOD_SECONDS: f32 = 2.;
const GRACE_PERIOD_SECONDS: f32 = 2.0;
const INITIAL_EXTEND_SCALE: f32 = 8.0;
const MINIMUM_EXTEND_SCALE: f32 = 1.0;

const SCALE_INCREASE_FACTOR: f32 = 1.2;
const SCALE_DECREASE_FACTOR: f32 = 0.8;
const COOLDOWN_INCREASE_FACTOR: f32 = 1.2;
const COOLDOWN_DECREASE_FACTOR: f32 = 0.8;

#[derive(Event)]
pub struct InputAttackEvent;

#[derive(Event)]
pub struct AttackDirection(pub Vec2);

#[derive(Component)]
pub struct Attack {
    /// Current phase of the attack
    pub phase: AttackPhase,
    /// Determines the timer length for reacting phase
    pub attack_delay: f32,
    /// Weapon size multiplier
    pub extend_scale: f32,
    pub attack_position: AttackPosition,
}

#[derive(Default)]
pub enum AttackPosition {
    #[default]
    Up,
    Down,
}

impl AttackPosition {
    pub fn vector(&self, attack_direction: Vec2) -> Vec2 {
        match self {
            AttackPosition::Up => Vec2::Y * attack_direction.y.signum(),
            AttackPosition::Down => Vec2::NEG_Y * attack_direction.y.signum(),
        }
    }
}

pub enum AttackPhase {
    /// Weapon is chain reacting, timer is how long from button press to attack
    Reacting(Timer),
    /// Attack animation time
    Attacking,
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
            AttackPhase::Attacking => {}
            AttackPhase::Ready(timer) => {
                timer.tick(time);
            }
            AttackPhase::Cooling(timer) => {
                timer.tick(time);
            }
        }
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
            attack_delay,
            extend_scale,
            attack_position: AttackPosition::default(),
        }
    }

    pub fn update_fury(&mut self, increase_fury: bool) {
        // that u128 duration cast to f64 should be fine
        // because it should never be bigger than INITIAL_ATTACK_COOLDOWN_MILLISECONDS
        if increase_fury {
            let decreased_cooldown = self.attack_delay * COOLDOWN_DECREASE_FACTOR;
            self.attack_delay = decreased_cooldown.max(MINIMUM_ATTACK_COOLDOWN_SECONDS);
            self.extend_scale =
                (self.extend_scale * SCALE_DECREASE_FACTOR).max(MINIMUM_EXTEND_SCALE);
        } else {
            let increased_cooldown = self.attack_delay * COOLDOWN_INCREASE_FACTOR;
            self.attack_delay = increased_cooldown.max(INITIAL_ATTACK_COOLDOWN_SECONDS);
            self.extend_scale =
                (self.extend_scale * SCALE_INCREASE_FACTOR).min(INITIAL_EXTEND_SCALE);
        };
    }

    pub fn new_reaction_timer(&self) -> AttackPhase {
        AttackPhase::Reacting(Timer::from_seconds(self.attack_delay, TimerMode::Once))
    }
}

fn attack_handler(
    mut player: Single<(Option<&mut Attack>, Entity), With<Player>>,
    mut input_event: EventReader<InputAttackEvent>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // Loop so we consume events and don't block. but also we don't really care how many events get triggered
    let has_attack_input = input_event.read().fold(false, |acc, _| acc || true);

    let (attack_component, entity) = &mut *player;
    let attack = match attack_component {
        Some(a) => a,
        None => {
            if has_attack_input {
                commands.entity(*entity).insert(Attack::default());
            }
            return;
        }
    };

    attack.phase.tick(time.delta());

    match &mut attack.phase {
        AttackPhase::Reacting(timer) => {
            if timer.just_finished() {
                attack.update_fury(true);
                attack.phase = AttackPhase::Attacking;
            }
        }
        // Attacking is handled by animation
        AttackPhase::Attacking => {}
        AttackPhase::Ready(timer) => {
            if timer.just_finished() {
                // if we are in ready phase, we can start cooling down
                attack.phase = AttackPhase::Cooling(Timer::from_seconds(
                    GRACE_PERIOD_SECONDS,
                    TimerMode::Once,
                ));
            } else if has_attack_input {
                attack.update_fury(true);
                attack.phase = attack.new_reaction_timer();
            }
        }
        AttackPhase::Cooling(timer) => {
            if timer.just_finished() {
                commands.entity(*entity).remove::<Attack>();
            } else if has_attack_input {
                attack.update_fury(false);
                attack.phase = attack.new_reaction_timer();
            }
        }
    }
}

fn do_attack(mut player: Single<(&mut Attack, Entity), With<Player>>) {
    let (attack, entity) = &mut *player;

    if let AttackPhase::Attacking = attack.phase {
        attack.phase =
            AttackPhase::Ready(Timer::from_seconds(ATTACK_PERIOD_SECONDS, TimerMode::Once));
    }
}

fn player_attack_direction(
    mut input_event: EventReader<AttackDirection>,
    mut player: Single<(&mut Player, Has<Grounded>)>,
) {
    let (p, is_grounded) = &mut *player;

    // note: this is only a vec2 because maybe we want diagonal attacks, but i lowkey regret making it like this now
    for AttackDirection(direction) in input_event.read() {
        let attack_dir = match direction {
            d if d.y > 0.0 => Vec2::Y,
            // only attack down if not grounded
            d if d.y < 0.0 && !*is_grounded => Vec2::NEG_Y,
            d if d.y < 0.0 && *is_grounded => continue,
            d if d.x > 0.0 => Vec2::X,
            d if d.x < 0.0 => Vec2::NEG_X,
            _ => continue,
        };
        p.attack_direction = attack_dir;
    }
}
