use avian2d::prelude::{Collider, ColliderDisabled};
use bevy::prelude::*;

use crate::{
    physics::creature::Grounded,
    player::{
        attack::{
            behaviour::{Attack, AttackDirection, AttackPhase, DoAttackEvent, InputAttackEvent},
            sound::{AttackAssets, AttackSound, play_attack_sound},
        },
        character::Player,
        input::{gamepad_attack_input, keyboard_attack_input},
        weapon::WEAPON_HITBOX_NAME,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            (
                keyboard_attack_input,
                gamepad_attack_input,
                player_attack_direction,
            ),
            (
                attack_handler,
                play_attack_sound.run_if(resource_exists::<AttackAssets>),
                do_attack,
            )
                .chain(),
        )
            .chain(),
    );
}

fn attack_handler(
    mut player: Single<(Option<&mut Attack>, Entity, &Transform, &Player)>,
    fuckin_cooliders: Query<(&Name, Entity, &Collider)>,
    mut input_event: EventReader<InputAttackEvent>,
    mut commands: Commands,
    time: Res<Time>,
    mut sound_event: EventWriter<AttackSound>,
) {
    // Loop so we consume events and don't block. but also we don't really care how many events get triggered
    let has_attack_input = input_event.read().fold(false, |acc, _| acc || true);

    let (attack_component, entity, transform, player) = &mut *player;
    let attack = match attack_component {
        Some(a) => a,
        None => {
            if has_attack_input {
                commands.entity(*entity).insert(Attack::default());
            }
            return;
        }
    };

    // unwrapping here because, i can't really imaging how this entity is just gone, we'd be really fucked
    let the_one_i_need = fuckin_cooliders
        .iter()
        .find(|(name, _, _)| name.contains(WEAPON_HITBOX_NAME))
        .unwrap()
        .1;

    attack.phase.tick(time.delta());

    match &mut attack.phase {
        AttackPhase::Reacting(timer) => {
            if timer.just_finished() {
                attack.phase = AttackPhase::Attacking {
                    pos: transform.translation,
                    direction: player.attack_direction,
                    didithit: None,
                };
                sound_event.write(AttackSound::Slash);
                commands.entity(the_one_i_need).remove::<ColliderDisabled>();
            }
        }
        // Attacking is handled by animation
        AttackPhase::Attacking {
            pos,
            direction,
            didithit,
        } => {}
        AttackPhase::Ready(timer) => {
            commands.entity(the_one_i_need).insert(ColliderDisabled);
            if timer.just_finished() {
                attack.update_fury(false);
                // if we are in ready phase, we can start cooling down
                attack.phase = AttackPhase::new_cooling_timer();
            } else if has_attack_input {
                // this should go when you move to the ready phase
                attack.phase = attack.new_reaction_timer();
            }
        }
        AttackPhase::Cooling(timer) => {
            if timer.just_finished() {
                commands.entity(*entity).remove::<Attack>();
            } else if has_attack_input {
                attack.phase = attack.new_reaction_timer();
            }
        }
    }
}

fn do_attack(
    mut player: Single<(&mut Attack, Entity), With<Player>>,
    mut do_attack_event: EventReader<DoAttackEvent>,
    mut play_sound_writer: EventWriter<AttackSound>,
) {
    let (attack, entity) = &mut *player;

    let is_attacking = matches!(attack.phase, AttackPhase::Attacking { .. });

    if is_attacking {
        for _ in do_attack_event.read() {
            // figure out how to get this
            let collision = true;
            let delay_seconds = attack.attack_delay_seconds;

            if collision {
                play_sound_writer.write(AttackSound::Hit(delay_seconds));
            } else {
                play_sound_writer.write(AttackSound::Miss(delay_seconds));
            }

            if let AttackPhase::Attacking { didithit, .. } = &mut attack.phase {
                *didithit = Some(collision);
            }
        }
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
