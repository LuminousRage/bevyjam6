//! Handling player movement control
//! Heavily referencing (aka plagiarising/copying)
//! https://github.com/Jondolf/avian/blob/main/crates/avian2d/examples/dynamic_character_2d/plugin.rs

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::{
    PausableSystems,
    physics::creature::CreaturePhysicsBundle,
    player::{
        character::Player,
        configs::{MAX_SLOPE_ANGLE, MOVEMENT_DAMPING, MOVEMENT_SPEED},
        input::{gamepad_movement_input, keyboard_movement_input},
        movement::{
            coyote::{detect_coyote_time_start, handle_coyote_time},
            dashing::{DashingEvent, handle_dash_event},
            jumping::{JumpingEvent, handle_jump_event},
            movement_visual::SpriteImageChange,
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
            (handle_jump_event, handle_dash_event),
        )
            .chain()
            .in_set(PausableSystems),),
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
}

impl PlayerMovementBundle {
    pub fn new(collider: Collider, scale: Vector) -> Self {
        Self {
            state: PlayerMovementState::Idle(false),
            physics: CreaturePhysicsBundle::new(collider, scale, MOVEMENT_DAMPING, MAX_SLOPE_ANGLE),
        }
    }
}

#[derive(Component, Reflect, Clone, PartialEq, Debug)]
#[reflect(Component)]
pub enum PlayerMovementState {
    /// bool is sprite reversing stuff
    Idle(bool),
    Run,
    Jump(Timer),
    Dash(f32),
}

fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut jump_event_writer: EventWriter<JumpingEvent>,
    mut dash_event_writer: EventWriter<DashingEvent>,
    mut sprite_change_event: EventWriter<SpriteImageChange>,
    controller: Single<(&mut Player, &mut LinearVelocity, &mut PlayerMovementState)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();
    let (mut player, mut linear_velocity, mut player_movement_state) = controller.into_inner();

    if movement_event_reader.is_empty() && *player_movement_state == PlayerMovementState::Run {
        sprite_change_event.write(SpriteImageChange(PlayerMovementState::Idle(false)));
        *player_movement_state = PlayerMovementState::Idle(false);
    }

    for event in movement_event_reader.read() {
        {
            match event {
                MovementAction::Move(direction) => {
                    if let PlayerMovementState::Dash(_) = *player_movement_state {
                        // we don't fuck with dashing
                        continue;
                    }

                    player.face_direction = *direction;
                    let desired_speed = direction.x * MOVEMENT_SPEED - linear_velocity.x;
                    linear_velocity.x += desired_speed * 10. * delta_time;

                    if let PlayerMovementState::Idle(_) = *player_movement_state {
                        sprite_change_event.write(SpriteImageChange(PlayerMovementState::Run));
                        *player_movement_state = PlayerMovementState::Run;
                    }
                }
                MovementAction::JumpStart => {
                    jump_event_writer.write(JumpingEvent { is_start: true });
                }
                MovementAction::JumpEnd => {
                    jump_event_writer.write(JumpingEvent { is_start: false });
                }
                MovementAction::Dash => {
                    dash_event_writer.write(DashingEvent { is_start: true });
                }
            }
        }
    }
}
