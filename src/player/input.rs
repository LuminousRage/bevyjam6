use avian2d::math::Scalar;
use bevy::prelude::*;

use crate::player::{
    attack::AttackDirection,
    configs::{KEYBOARD_DOWN, KEYBOARD_UP},
};

use super::{
    attack::InputAttackEvent,
    configs::{KEYBOARD_ATTACK, KEYBOARD_DASH, KEYBOARD_JUMP, KEYBOARD_LEFT, KEYBOARD_RIGHT},
    movement::MovementAction,
};

fn input_to_direction(left: bool, right: bool, up: bool, down: bool) -> Option<Vec2> {
    let horizontal_movement = (right as i8 - left as i8) as Scalar;
    let vertical_movement = (up as i8 - down as i8) as Scalar;

    if horizontal_movement == 0.0 && vertical_movement == 0.0 {
        return None;
    }
    Some(Vec2::new(horizontal_movement, vertical_movement).normalize_or_zero())
}

/// Sends [`MovementAction`] events based on keyboard input.
pub fn keyboard_movement_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let direction = {
        let left = keyboard_input.pressed(KEYBOARD_LEFT);
        let right = keyboard_input.pressed(KEYBOARD_RIGHT);
        let up = keyboard_input.pressed(KEYBOARD_UP);
        let down = keyboard_input.pressed(KEYBOARD_DOWN);
        input_to_direction(left, right, up, down)
    };

    if let Some(direction) = direction.filter(|d| d.x != 0.0) {
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
        let direction = {
            let axis_direction = {
                let left = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
                let up = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);

                if left == 0.0 && up == 0.0 {
                    None
                } else {
                    Some(Vec2::new(left, up).normalize_or_zero())
                }
            };
            let button_direction = {
                let left = gamepad.pressed(GamepadButton::DPadLeft);
                let right = gamepad.pressed(GamepadButton::DPadRight);
                let up = gamepad.pressed(GamepadButton::DPadUp);
                let down = gamepad.pressed(GamepadButton::DPadDown);
                input_to_direction(left, right, up, down)
            };
            button_direction.or(axis_direction)
        };

        if let Some(d) = direction.filter(|d| d.x != 0.0) {
            movement_event_writer.write(MovementAction::Move(d));
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
    mut attack_direction_writer: EventWriter<AttackDirection>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // duplicate code i know, but because of dependencies/chaining, i wouldn't want to clean this up before knowing
    // 100% that everything is alright.
    let direction = {
        let left = keyboard_input.pressed(KEYBOARD_LEFT);
        let right = keyboard_input.pressed(KEYBOARD_RIGHT);
        let up = keyboard_input.pressed(KEYBOARD_UP);
        let down = keyboard_input.pressed(KEYBOARD_DOWN);
        input_to_direction(left, right, up, down)
    };

    if let Some(d) = direction.filter(|d| d.x != 0.0) {
        attack_direction_writer.write(AttackDirection(d));
    }

    if keyboard_input.pressed(KEYBOARD_ATTACK) {
        attack_event_writer.write(InputAttackEvent);
    }
}

pub fn gamepad_attack_input(
    mut attack_event_writer: EventWriter<InputAttackEvent>,
    mut attack_direction_writer: EventWriter<AttackDirection>,
    gamepads: Query<&Gamepad>,
) {
    for gamepad in gamepads.iter() {
        let direction = {
            let axis_direction = {
                let left = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
                let up = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);

                if left == 0.0 && up == 0.0 {
                    None
                } else {
                    Some(Vec2::new(left, up).normalize_or_zero())
                }
            };
            let button_direction = {
                let left = gamepad.pressed(GamepadButton::DPadLeft);
                let right = gamepad.pressed(GamepadButton::DPadRight);
                let up = gamepad.pressed(GamepadButton::DPadUp);
                let down = gamepad.pressed(GamepadButton::DPadDown);
                input_to_direction(left, right, up, down)
            };
            button_direction.or(axis_direction)
        };

        if let Some(d) = direction.filter(|d| d.x != 0.0) {
            attack_direction_writer.write(AttackDirection(d));
        }

        if gamepad.pressed(GamepadButton::West) {
            attack_event_writer.write(InputAttackEvent);
        }
    }
}
