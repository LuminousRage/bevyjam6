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

#[derive(Event)]
pub struct InputAttackEvent;

fn attack_input(
    mut attack_event: EventReader<InputAttackEvent>,
    mut commands: Commands,
    player_entity: Single<(Entity, Has<Attack>), With<Player>>,
) {
    for event in attack_event.read() {
        let (entity, has_attack) = *player_entity;

        if !has_attack {
            commands.entity(entity).insert(Attack {
                cooldown: Timer::new(
                    Duration::from_millis(INITIAL_ATTACK_COOLDOWN_MILLISECONDS),
                    TimerMode::Once,
                ),
            });
        }
    }
}

#[derive(Component)]
pub struct Attack {
    cooldown: Timer,
}

fn handle_attack(attack: Option<Single<&mut Attack, With<Player>>>) {}
