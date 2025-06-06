use std::time::Duration;

use bevy::prelude::*;

use crate::physics::creature::Grounded;

use super::{
    character::Player,
    input::{gamepad_attack_input, keyboard_attack_input},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<InputAttackEvent>();
    app.add_event::<DoAttackEvent>();
    app.add_event::<AttackDirection>();
    app.add_systems(
        Update,
        (
            (
                keyboard_attack_input,
                gamepad_attack_input,
                player_attack_direction,
            ),
            (attack_kickstart, attack_timer_handler, do_attack).chain(),
        )
            .chain(),
    );
}

const INITIAL_ATTACK_COOLDOWN_MILLISECONDS: u64 = 2000;
const MINIMUM_ATTACK_COOLDOWN_MILLISECONDS: u64 = 100;
const ATTACK_PERIOD_MILLISECONDS: u64 = 200;
const INITIAL_EXTEND_SCALE: f32 = 8.0;
const MINIMUM_EXTEND_SCALE: f32 = 1.0;

const SCALE_INCREASE_FACTOR: f32 = 1.2;
const SCALE_DECREASE_FACTOR: f32 = 0.8;
const COOLDOWN_INCREASE_FACTOR: f64 = 1.2;
const COOLDOWN_DECREASE_FACTOR: f64 = 0.8;

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
    pub attack_delay: f32,
    /// Weapon size multiplier
    pub extend_scale: f32,
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
            INITIAL_ATTACK_COOLDOWN_MILLISECONDS as f32 / 1000.0,
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
        Attack::new(INITIAL_ATTACK_COOLDOWN_MILLISECONDS, INITIAL_EXTEND_SCALE)
    }
}
impl Attack {
    pub fn new(initial_attack_cooldown: u64, extend_scale: f32) -> Self {
        Self {
            phase: AttackPhase::default(),
            attack_delay: initial_attack_cooldown as f32,
            extend_scale,
        }
    }

    // Triggers an attack action - this should decrease the cooldown and reset everything.
    // pub fn update_cooldown_timer(&mut self, decrease_cooldown: bool) {
    //     // that u128 duration cast to f64 should be fine
    //     // because it should never be bigger than INITIAL_ATTACK_COOLDOWN_MILLISECONDS
    //     let current_cooldown = self.attack_delay.duration().as_millis() as f64;

    //     let (new_cooldown, new_extend_scale) = if decrease_cooldown {
    //         let decreased_cooldown = current_cooldown * COOLDOWN_DECREASE_FACTOR;
    //         let new_cooldown =
    //             (decreased_cooldown.round() as u64).max(MINIMUM_ATTACK_COOLDOWN_MILLISECONDS);
    //         let new_extend_scale =
    //             (self.extend_scale * SCALE_DECREASE_FACTOR).max(MINIMUM_EXTEND_SCALE);
    //         (new_cooldown, new_extend_scale)
    //     } else {
    //         let increased_cooldown = current_cooldown * COOLDOWN_INCREASE_FACTOR;
    //         let new_cooldown =
    //             (increased_cooldown.round() as u64).min(INITIAL_ATTACK_COOLDOWN_MILLISECONDS);
    //         let new_extend_scale =
    //             (self.extend_scale * SCALE_INCREASE_FACTOR).min(INITIAL_EXTEND_SCALE);
    //         (new_cooldown, new_extend_scale)
    //     };
    //     *self = Attack::new(new_cooldown, new_extend_scale, false);
    // }

    // /// Tick cooldown timer if not finished, otherwise tick attack period
    // pub fn tick(&mut self, delta: Duration) {
    //     if self.attack_delay.finished() {
    //         self.attack_tolerance.tick(delta);
    //     } else {
    //         self.attack_delay.tick(delta);
    //     }
    // }
}

// i think separating might help with testing
/// Initialise attack component for idle players
fn attack_kickstart(
    mut commands: Commands,
    mut input_event: EventReader<InputAttackEvent>,
    mut attack_event: EventWriter<DoAttackEvent>,
    player: Single<Entity, (With<Player>, Without<Attack>)>,
) {
    // Consume events so we don't block. but also we don't really care how many events get triggered
    let mut has_attack_input = false;
    for _ in input_event.read() {
        has_attack_input = true;
    }

    if has_attack_input {
        commands.entity(*player).insert(Attack::default());
        attack_event.write(DoAttackEvent);
    }
}

fn attack_timer_handler(
    mut player: Single<(&mut Attack, Entity), With<Player>>,
    mut input_event: EventReader<InputAttackEvent>,
    mut attack_event: EventWriter<DoAttackEvent>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let (attack, entity) = &mut *player;
    attack.phase.tick(time.delta());

    // Consume events so we don't block. but also we don't really care how many events get triggered
    let mut has_attack_input = false;
    for _ in input_event.read() {
        has_attack_input = true;
    }

    // match (
    //     attack.attack_delay.finished(),
    //     attack.attack_tolerance.just_finished(),
    //     attack.previous_attack_failed,
    // ) {
    //     // in cooldown period, just ignore
    //     (false, _, _) => {
    //         if attack.attack_tolerance.just_finished() || attack.previous_attack_failed {
    //             dbg!("Attack period finished while in cooldown, this should not happen");
    //         }
    //     }

    //     // cooldown finished, we are in attack period
    //     (true, false, status) => {
    //         if has_attack_input {
    //             attack.update_cooldown_timer(!status);
    //             attack_event.write(DoAttackEvent);
    //         }
    //         // no attack, gg go next
    //     }

    //     // first offence
    //     (true, true, false) => {
    //         attack.previous_attack_failed = true;
    //         // handle weapon size
    //     }

    //     // second offence, remove pls
    //     (true, true, true) => {
    //         commands.entity(*entity).remove::<Attack>();
    //         // handle weapon size
    //     }
    // }
}

fn do_attack(mut attack_event: EventReader<DoAttackEvent>) {
    for _ in attack_event.read() {}
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
