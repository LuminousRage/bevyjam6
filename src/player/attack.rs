use std::time::Duration;

use bevy::prelude::*;

use super::{
    character::Player,
    input::{gamepad_attack_input, keyboard_attack_input},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<InputAttackEvent>();
    app.add_systems(
        Update,
        ((keyboard_attack_input, gamepad_attack_input), attack_input).chain(),
    );
}

const INITIAL_ATTACK_COOLDOWN_MILLISECONDS: u64 = 1000;
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

    /// Checks if the player should remove the component and reset to idle.
    pub fn should_reset(&self) -> bool {
        self.cooldown.finished() && self.attack_period.finished() && self.previous_attack_failed
    }

    pub fn update_failed_attack(&mut self) {
        if self.cooldown.finished() && self.attack_period.finished() {
            self.previous_attack_failed = true;
        }
    }

    /// Triggers an attack action - this should decrease the cooldown and reset everything.
    pub fn trigger(&mut self) {
        // that u128 duration cast to f64 should be fine
        // because it should never be bigger than INITIAL_ATTACK_COOLDOWN_MILLISECONDS
        let decreased_cooldown =
            self.cooldown.duration().as_millis() as f64 * COOLDOWN_INCREASE_FACTOR;
        let new_cooldown =
            (decreased_cooldown.round() as u64).max(MINIMUM_ATTACK_COOLDOWN_MILLISECONDS);

        *self = Attack::new(new_cooldown, false);
    }

    /// We can attack now
    pub fn in_attack_time(&self) -> bool {
        self.cooldown.finished() && !self.attack_period.finished()
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

fn attack_timer_handler(
    mut player: Single<(&mut Attack, Entity), With<Player>>,
    mut input_event: EventReader<InputAttackEvent>,
    mut attack_event: EventWriter<DoAttackEvent>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let (attack, entity) = &mut *player;
    attack.tick(time.delta());

    for _ in input_event.read() {
        if attack.in_attack_time() {
            attack.trigger();
            attack_event.write(DoAttackEvent);
            // handle weapon size
        }
    }

    if attack.should_reset() {
        commands.entity(*entity).remove::<Attack>();
    }

    attack.update_failed_attack();
}

fn attack_input(
    mut input_event: EventReader<InputAttackEvent>,
    mut attack_event: EventWriter<DoAttackEvent>,
    mut commands: Commands,
    player_entity: Single<(Entity, Has<Attack>), With<Player>>,
) {
    for _ in input_event.read() {
        let (entity, has_attack) = *player_entity;

        // player was idle, start new attack
        if !has_attack {
            commands.entity(entity).insert(Attack::default());

            attack_event.write(DoAttackEvent);
        }
    }
}

fn handle_attack(attack: Option<Single<&mut Attack, With<Player>>>) {}
