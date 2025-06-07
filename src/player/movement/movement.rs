//! Handling player movement control
//! Heavily referencing (aka plagiarising/copying)
//! https://github.com/Jondolf/avian/blob/main/crates/avian2d/examples/dynamic_character_2d/plugin.rs

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::{
    physics::creature::{CreaturePhysicsBundle, Flying},
    player::{
        character::Player,
        configs::{
            DASH_COOLDOWN_DURATION, DASH_DURATION, DASH_SPEED_MODIFIER, MAX_SLOPE_ANGLE,
            MOVEMENT_DAMPING, MOVEMENT_SPEED,
        },
        input::{gamepad_movement_input, keyboard_movement_input},
        movement::{
            coyote::{detect_coyote_time_start, handle_coyote_time},
            dashing::Dashing,
            jumping::{JumpingEvent, handle_jump_event},
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<MovementAction>().add_systems(
        Update,
        ((
            (
                keyboard_movement_input,
                gamepad_movement_input,
                detect_coyote_time_start,
                handle_coyote_time,
            ),
            movement,
            handle_jump_event,
        )
            .chain(),),
    );

    app.register_type::<PlayerMovementState>();
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vec2),
    JumpStart,
    JumpEnd,
    Dash,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct PlayerMovementBundle {
    state: PlayerMovementState,
    physics: CreaturePhysicsBundle,
    dashing: Dashing,
}

impl PlayerMovementBundle {
    pub fn new(collider: Collider, scale: Vector) -> Self {
        Self {
            state: PlayerMovementState::Idle,
            physics: CreaturePhysicsBundle::new(collider, scale, MOVEMENT_DAMPING, MAX_SLOPE_ANGLE),
            dashing: Dashing::new(),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum PlayerMovementState {
    Idle,
    Run,
    Jump(Timer),
    Dash,
}

fn movement(
    time: Res<Time>,
    mut commands: Commands,
    mut movement_event_reader: EventReader<MovementAction>,
    mut jump_event_writer: EventWriter<JumpingEvent>,
    mut controllers: Query<(
        Entity,
        &mut Player,
        &mut LinearVelocity,
        &mut Dashing,
        &mut GravityScale,
        &PlayerMovementState,
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (
            entity,
            mut player,
            mut linear_velocity,
            mut dashing,
            mut gravity,
            player_movement_state,
        ) in &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    if let PlayerMovementState::Dash = player_movement_state {
                        // we don't fuck with dashing
                        continue;
                    }

                    player.face_direction = *direction;
                    let desired_speed = direction.x * MOVEMENT_SPEED - linear_velocity.x;
                    linear_velocity.x += desired_speed * 10. * delta_time;
                }
                MovementAction::JumpStart => {
                    jump_event_writer.write(JumpingEvent { is_start: true });
                }
                MovementAction::JumpEnd => {
                    jump_event_writer.write(JumpingEvent { is_start: false });
                }
                MovementAction::Dash => {
                    if dashing.used
                        || dashing.current_cooldown > 0.0
                        || dashing.current_duration > 0.0
                    {
                        // Can't use dash, do nothing
                        continue;
                    }
                    commands.entity(entity).insert(Flying);
                    linear_velocity.x =
                        player.face_direction.x * MOVEMENT_SPEED * DASH_SPEED_MODIFIER;
                    linear_velocity.y = 0.0;
                    gravity.0 = 0.0;
                    dashing.current_cooldown = DASH_COOLDOWN_DURATION;
                    dashing.current_duration = DASH_DURATION;
                    dashing.used = true;
                }
            }
        }
    }
}
