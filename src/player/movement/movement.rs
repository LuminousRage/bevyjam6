//! Handling player movement control
//! Heavily referencing (aka plagiarising/copying)
//! https://github.com/Jondolf/avian/blob/main/crates/avian2d/examples/dynamic_character_2d/plugin.rs

use std::time::Duration;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::{
    physics::creature::{CreaturePhysicsBundle, Flying, Grounded},
    player::{
        character::Player,
        configs::{
            CHARACTER_GRAVITY_SCALE, DASH_COOLDOWN_DURATION, DASH_DURATION, DASH_SPEED_MODIFIER,
            JUMP_DURATION_MILLISECONDS, JUMP_IMPULSE, MAX_SLOPE_ANGLE, MOVEMENT_DAMPING,
            MOVEMENT_SPEED,
        },
        input::{gamepad_movement_input, keyboard_movement_input},
        movement::coyote::{Coyote, detect_coyote_time_start, handle_coyote_time},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<MovementAction>().add_systems(
        Update,
        (
            (
                (
                    keyboard_movement_input,
                    gamepad_movement_input,
                    detect_coyote_time_start,
                    handle_coyote_time,
                ),
                movement,
            )
                .chain(),
            handle_dashing,
            handle_jump_end,
        ),
    );
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vec2),
    JumpStart,
    JumpEnd,
    Dash,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Dashing {
    current_duration: f32,
    current_cooldown: f32,
    used: bool,
}

impl Dashing {
    fn new() -> Dashing {
        Self {
            current_duration: 0.0,
            current_cooldown: 0.0,
            used: false,
        }
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Jumping {
    duration: Timer,
    cooldown: Timer,
}

impl Jumping {
    fn new(duration: u64) -> Jumping {
        Self {
            duration: Timer::new(Duration::from_millis(duration), TimerMode::Once),
            cooldown: Timer::new(Duration::from_millis(100), TimerMode::Once),
        }
    }
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    physics: CreaturePhysicsBundle,
    dashing: Dashing,
}

impl MovementBundle {
    pub fn new(collider: Collider, scale: Vector) -> Self {
        Self {
            physics: CreaturePhysicsBundle::new(collider, scale, MOVEMENT_DAMPING, MAX_SLOPE_ANGLE),
            dashing: Dashing::new(),
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
/// TODO: maybe break this up
fn movement(
    time: Res<Time>,
    mut commands: Commands,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        Entity,
        &mut Player,
        &mut LinearVelocity,
        Has<Grounded>,
        &mut Dashing,
        Has<Coyote>,
        &mut GravityScale,
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
            is_grounded,
            mut dashing,
            is_coyote,
            mut gravity,
        ) in &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    if dashing.current_duration > 0.0 {
                        continue;
                    }
                    player.face_direction = *direction;
                    let desired_speed = direction.x * MOVEMENT_SPEED - linear_velocity.x;
                    linear_velocity.x += desired_speed * 10. * delta_time;
                }
                MovementAction::JumpStart => {
                    if is_grounded || is_coyote {
                        commands.entity(entity).remove::<Grounded>();
                        commands.entity(entity).remove::<Coyote>();

                        commands
                            .entity(entity)
                            .insert(Jumping::new(JUMP_DURATION_MILLISECONDS));
                        linear_velocity.y += JUMP_IMPULSE;
                        gravity.0 = 0.5;
                    }
                }
                MovementAction::JumpEnd => {
                    // is in air and is going up
                    if !is_grounded && linear_velocity.y > 0.0 {
                        commands.entity(entity).remove::<Jumping>();
                        gravity.0 = CHARACTER_GRAVITY_SCALE;
                        linear_velocity.y *= 0.5;
                    }
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

fn handle_jump_end(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Jumping, &mut GravityScale, &mut LinearVelocity)>,
) {
    for (entity, mut jumping, mut gravity_scale, mut linear_velocity) in &mut query {
        jumping.duration.tick(time.delta());

        if jumping.duration.just_finished() {
            commands.entity(entity).remove::<Jumping>();
            gravity_scale.0 = CHARACTER_GRAVITY_SCALE;
            linear_velocity.y *= 0.5;
        }
    }
}

// maybe use an event for this, so collisions/damage can cancel dash
fn handle_dashing(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Dashing,
        &mut GravityScale,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    for (entity, mut dashing, mut gravity_scale, mut linear_velocity, is_grounded) in &mut query {
        let delta = time.delta().as_secs_f32().adjust_precision();
        dashing.current_cooldown -= delta;
        dashing.current_duration -= delta;

        if dashing.current_duration <= 0.0 && dashing.current_duration + delta > 0.0 {
            commands.entity(entity).remove::<Flying>();
            gravity_scale.0 = CHARACTER_GRAVITY_SCALE;
            linear_velocity.x *= 0.4;
        }

        if is_grounded {
            dashing.used = false;
        }
    }
}
