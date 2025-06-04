use avian2d::math::Scalar;
use bevy::prelude::*;

use super::{
    attack::InputAttackEvent,
    configs::{KEYBOARD_ATTACK, KEYBOARD_DASH, KEYBOARD_JUMP, KEYBOARD_LEFT, KEYBOARD_RIGHT},
    movement::MovementAction,
};

fn horizontal_input_to_direction(left: bool, right: bool) -> Scalar {
    (right as i8 - left as i8) as Scalar
}

/// Sends [`MovementAction`] events based on keyboard input.
pub fn keyboard_movement_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let direction = {
        let left = keyboard_input.any_pressed([KEYBOARD_LEFT]);
        let right = keyboard_input.any_pressed([KEYBOARD_RIGHT]);
        horizontal_input_to_direction(left, right)
    };

    if direction != 0.0 {
        movement_event_writer.write(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KEYBOARD_JUMP) {
        movement_event_writer.write(MovementAction::JumpStart);
    }
    if keyboard_input.just_released(KEYBOARD_JUMP) {
        movement_event_writer.write(MovementAction::JumpEnd);
    }

    if keyboard_input.just_pressed(KEYBOARD_DASH) {
        movement_event_writer.write(MovementAction::Dash);
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
pub fn gamepad_movement_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Query<&Gamepad>,
) {
    for gamepad in gamepads.iter() {
        if let Some(x) = gamepad.get(GamepadAxis::LeftStickX) {
            movement_event_writer.write(MovementAction::Move(x as Scalar));
        }

        let direction = {
            let left = gamepad.any_pressed([GamepadButton::DPadLeft]);
            let right = gamepad.any_pressed([GamepadButton::DPadRight]);
            horizontal_input_to_direction(left, right)
        };

        if direction != 0.0 {
            movement_event_writer.write(MovementAction::Move(direction));
        }

        if gamepad.just_pressed(GamepadButton::South) {
            movement_event_writer.write(MovementAction::JumpStart);
        }

        if gamepad.any_just_pressed([GamepadButton::RightTrigger, GamepadButton::RightTrigger2]) {
            movement_event_writer.write(MovementAction::JumpEnd);
        }
    }
}

pub fn keyboard_attack_input(
    mut attack_event_writer: EventWriter<InputAttackEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KEYBOARD_ATTACK) {
        attack_event_writer.write(InputAttackEvent);
    }
}

pub fn gamepad_attack_input(
    mut attack_event_writer: EventWriter<InputAttackEvent>,
    gamepads: Query<&Gamepad>,
) {
    for gamepad in gamepads.iter() {
        if gamepad.pressed(GamepadButton::West) {
            attack_event_writer.write(InputAttackEvent);
        }
    }
}
