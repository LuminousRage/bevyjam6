use std::time::Duration;

use bevy::prelude::*;

use super::{
    character::Player,
    input::{gamepad_attack_input, keyboard_attack_input},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<InputAttackEvent>();
    app.add_event::<DoAttackEvent>();
    app.add_systems(
        Update,
        (
            (keyboard_attack_input, gamepad_attack_input),
            (attack_kickstart, attack_timer_handler, do_attack).chain(),
        )
            .chain(),
    );
}

const INITIAL_ATTACK_COOLDOWN_MILLISECONDS: u64 = 2000;
const MINIMUM_ATTACK_COOLDOWN_MILLISECONDS: u64 = 100;
const ATTACK_PERIOD_MILLISECONDS: u64 = 200;

const COOLDOWN_INCREASE_FACTOR: f64 = 1.2;
const COOLDOWN_DECREASE_FACTOR: f64 = 0.8;

#[derive(Event)]
pub struct InputAttackEvent;

#[derive(Event)]
pub struct DoAttackEvent;

#[derive(Component)]
pub struct Attack {
    pub cooldown: Timer,
    pub attack_period: Timer,
    pub previous_attack_failed: bool,
}

impl Default for Attack {
    fn default() -> Self {
        Attack::new(INITIAL_ATTACK_COOLDOWN_MILLISECONDS, false)
    }
}
impl Attack {
    pub fn new(initial_attack_cooldown: u64, previous_attack_failed: bool) -> Self {
        Self {
            cooldown: Timer::new(
                Duration::from_millis(initial_attack_cooldown),
                TimerMode::Once,
            ),
            attack_period: Timer::new(
                Duration::from_millis(ATTACK_PERIOD_MILLISECONDS),
                TimerMode::Repeating,
            ),
            previous_attack_failed,
        }
    }

    /// Triggers an attack action - this should decrease the cooldown and reset everything.
    pub fn update_cooldown_timer(&mut self, decrease_cooldown: bool) {
        // that u128 duration cast to f64 should be fine
        // because it should never be bigger than INITIAL_ATTACK_COOLDOWN_MILLISECONDS
        let current_cooldown = self.cooldown.duration().as_millis() as f64;

        let new_cooldown = if decrease_cooldown {
            let decreased_cooldown = current_cooldown * COOLDOWN_DECREASE_FACTOR;
            (decreased_cooldown.round() as u64).max(MINIMUM_ATTACK_COOLDOWN_MILLISECONDS)
        } else {
            let increased_cooldown = current_cooldown * COOLDOWN_INCREASE_FACTOR;
            (increased_cooldown.round() as u64).min(INITIAL_ATTACK_COOLDOWN_MILLISECONDS)
        };

        *self = Attack::new(new_cooldown, false);
    }

    /// Tick cooldown timer if not finished, otherwise tick attack period
    pub fn tick(&mut self, delta: Duration) {
        if self.cooldown.finished() {
            self.attack_period.tick(delta);
        } else {
            self.cooldown.tick(delta);
        }
    }
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
        // handle weapon size
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
    attack.tick(time.delta());

    // Consume events so we don't block. but also we don't really care how many events get triggered
    let mut has_attack_input = false;
    for _ in input_event.read() {
        has_attack_input = true;
    }

    match (
        attack.cooldown.finished(),
        attack.attack_period.just_finished(),
        attack.previous_attack_failed,
    ) {
        // in cooldown period, just ignore
        (false, _, _) => {
            if attack.attack_period.just_finished() || attack.previous_attack_failed {
                dbg!("Attack period finished while in cooldown, this should not happen");
            }
        }

        // cooldown finished, we are in attack period
        (true, false, status) => {
            if has_attack_input {
                attack.update_cooldown_timer(status);
                attack_event.write(DoAttackEvent);
                // handle weapon size
            }
            // no attack, gg go next
        }

        // first offence
        (true, true, false) => {
            attack.previous_attack_failed = true;
            // handle weapon size
        }

        // second offence, remove pls
        (true, true, true) => {
            commands.entity(*entity).remove::<Attack>();
            // handle weapon size
        }
    }
}

fn do_attack(mut attack_event: EventReader<DoAttackEvent>) {
    for _ in attack_event.read() {}
}
