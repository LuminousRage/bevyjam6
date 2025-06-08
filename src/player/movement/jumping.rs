use avian2d::prelude::{GravityScale, LinearVelocity};
use bevy::prelude::*;

use crate::{
    PausableSystems,
    physics::creature::Grounded,
    player::{
        character::Player,
        configs::{CHARACTER_GRAVITY_SCALE, JUMP_DURATION_SECONDS, JUMP_IMPULSE},
        movement::{
            coyote::Coyote, movement::PlayerMovementState, movement_visual::SpriteImageChange,
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<JumpingEvent>();
    app.add_systems(Update, handle_jump_timer.in_set(PausableSystems));
}

#[derive(Event)]
pub struct JumpingEvent {
    /// false cancels the jump
    pub is_start: bool,
}

pub fn handle_jump_event(
    player: Single<
        (
            Entity,
            &mut LinearVelocity,
            &mut GravityScale,
            &mut PlayerMovementState,
            Has<Grounded>,
            Has<Coyote>,
        ),
        With<Player>,
    >,
    mut jump_event_reader: EventReader<JumpingEvent>,
    mut sprite_change_event: EventWriter<SpriteImageChange>,
    mut commands: Commands,
) {
    let (entity, mut linear_velocity, mut gravity, mut movement_state, is_grounded, is_coyote) =
        player.into_inner();

    for event in jump_event_reader.read() {
        match event.is_start {
            true => {
                if let PlayerMovementState::Dash(_) = *movement_state {
                    continue;
                }

                if is_grounded || is_coyote {
                    commands.entity(entity).remove::<Grounded>();
                    commands.entity(entity).remove::<Coyote>();
                    *movement_state = PlayerMovementState::Jump(Timer::from_seconds(
                        JUMP_DURATION_SECONDS,
                        TimerMode::Once,
                    ));
                    sprite_change_event.write(SpriteImageChange(movement_state.clone()));
                    linear_velocity.y += JUMP_IMPULSE;
                    gravity.0 = 0.5;
                }
            }
            false => {
                if !is_grounded && linear_velocity.y > 0.0 {
                    gravity.0 = CHARACTER_GRAVITY_SCALE;
                    linear_velocity.y *= 0.5;
                }
            }
        }
    }
}

fn handle_jump_timer(
    time: Res<Time>,
    jumping: Single<(&mut PlayerMovementState, Has<Grounded>), With<Player>>,
    mut jump_event_writer: EventWriter<JumpingEvent>,
    mut sprite_change_event: EventWriter<SpriteImageChange>,
) {
    let (mut state, is_grounded) = jumping.into_inner();
    if let PlayerMovementState::Jump(timer) = &mut *state {
        timer.tick(time.delta());
        if timer.just_finished() {
            jump_event_writer.write(JumpingEvent { is_start: false });
        }

        if is_grounded {
            *state = PlayerMovementState::Idle(false);
            sprite_change_event.write(SpriteImageChange(state.clone()));
        }
    }
}
