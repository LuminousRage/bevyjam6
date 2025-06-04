//! Handling player movement control
//! Heavily referencing (aka plagiarising/copying)
//! https://github.com/Jondolf/avian/blob/main/crates/avian2d/examples/dynamic_character_2d/plugin.rs

use std::time::Duration;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::physics::creature::{CreaturePhysicsBundle, Flying, Grounded};

use super::configs::{
    CHARACTER_GRAVITY_SCALE, DASH_DURATION_MILLISECONDS, DASH_SPEED_MODIFIER,
    JUMP_DURATION_MILLISECONDS, JUMP_IMPULSE, MAX_SLOPE_ANGLE, MOVEMENT_ACCELERATION,
    MOVEMENT_DAMPING,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<MovementAction>().add_systems(
        Update,
        (
            keyboard_input,
            gamepad_input,
            detect_coyote_time_start,
            handle_coyote_time,
            handle_dashing,
            handle_jump_end,
            movement,
        ),
    );
    app.register_type::<JumpImpulse>()
        .register_type::<MovementAcceleration>();
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    JumpStart,
    JumpEnd,
    Dash,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Dashing(Timer);

impl Dashing {
    fn new(duration: u64) -> Dashing {
        Self(Timer::new(Duration::from_millis(duration), TimerMode::Once))
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Jumping(Timer);

impl Jumping {
    fn new(duration: u64) -> Jumping {
        Self(Timer::new(Duration::from_millis(duration), TimerMode::Once))
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Coyote(Timer);

impl Coyote {
    fn new(duration: u64) -> Coyote {
        Self(Timer::new(Duration::from_millis(duration), TimerMode::Once))
    }
}

/// The acceleration used for character movement.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementAcceleration(Scalar);

/// The strength of a jump.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct JumpImpulse(Scalar);

/// The direction the player is facing.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerFaceDirection(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle, Reflect)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    jump_impulse: JumpImpulse,
    player_face_direction: PlayerFaceDirection,
    physics: CreaturePhysicsBundle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            jump_impulse: JumpImpulse(jump_impulse),
            physics: CreaturePhysicsBundle::new(damping, max_slope_angle),
            player_face_direction: PlayerFaceDirection(1.0),
        }
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(10.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::new(
                MOVEMENT_ACCELERATION,
                MOVEMENT_DAMPING,
                JUMP_IMPULSE,
                MAX_SLOPE_ANGLE,
            ),
        }
    }
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as Scalar;

    if direction != 0.0 {
        movement_event_writer.write(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        movement_event_writer.write(MovementAction::JumpStart);
    }
    if keyboard_input.just_released(KeyCode::KeyZ) {
        movement_event_writer.write(MovementAction::JumpEnd);
    }

    if keyboard_input.just_pressed(KeyCode::KeyC) {
        movement_event_writer.write(MovementAction::Dash);
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Query<&Gamepad>,
) {
    for gamepad in gamepads.iter() {
        if let Some(x) = gamepad.get(GamepadAxis::LeftStickX) {
            movement_event_writer.write(MovementAction::Move(x as Scalar));
        }

        if gamepad.just_pressed(GamepadButton::South) {
            movement_event_writer.write(MovementAction::JumpStart);
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
        &MovementAcceleration,
        &JumpImpulse,
        &mut PlayerFaceDirection,
        &mut LinearVelocity,
        Has<Grounded>,
        Has<Dashing>,
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
            movement_acceleration,
            jump_impulse,
            mut player_direction,
            mut linear_velocity,
            is_grounded,
            is_dashing,
            is_coyote,
            mut gravity,
        ) in &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    if is_dashing {
                        continue;
                    }
                    player_direction.0 = *direction;

                    linear_velocity.x += *direction * movement_acceleration.0 * delta_time;
                }
                MovementAction::JumpStart => {
                    if is_grounded || is_coyote {
                        commands
                            .entity(entity)
                            .insert(Jumping::new(JUMP_DURATION_MILLISECONDS));
                        linear_velocity.y += jump_impulse.0;
                        gravity.0 = 1.0;
                    }
                }
                MovementAction::JumpEnd => {
                    // is in air and is going up
                    commands.entity(entity).remove::<Jumping>();
                    if !is_grounded && linear_velocity.y > 0.0 {
                        gravity.0 = CHARACTER_GRAVITY_SCALE;
                        linear_velocity.y *= 0.2;
                    }
                }
                MovementAction::Dash => {
                    if is_dashing {
                        // Already dashing, do nothing
                        continue;
                    }
                    commands
                        .entity(entity)
                        .insert(Dashing::new(DASH_DURATION_MILLISECONDS))
                        .insert(Flying);
                    linear_velocity.x += player_direction.0
                        * movement_acceleration.0
                        * DASH_SPEED_MODIFIER
                        * delta_time;
                    linear_velocity.y = 0.0;
                    gravity.0 = 0.0;
                }
            }
        }
    }
}

pub fn detect_coyote_time_start(
    query: Query<Entity, (With<CharacterController>, With<Grounded>, Without<Coyote>)>,
    mut commands: Commands,
) {
    for entity in query {
        commands.entity(entity).insert(Coyote::new(200));
    }
}

fn handle_coyote_time(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Coyote), Without<Grounded>>,
) {
    for (entity, mut coyote) in &mut query {
        coyote.0.tick(time.delta());

        if coyote.0.finished() {
            commands.entity(entity).remove::<Coyote>();
        }
    }
}

fn handle_jump_end(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Jumping, &mut GravityScale, &mut LinearVelocity)>,
) {
    for (entity, mut jumping, mut gravity_scale, mut linear_velocity) in &mut query {
        jumping.0.tick(time.delta());

        if jumping.0.finished() {
            commands.entity(entity).remove::<Jumping>();
            gravity_scale.0 = CHARACTER_GRAVITY_SCALE;
            linear_velocity.y *= 0.2;
        }
    }
}

// maybe use an event for this, so collisions/damage can cancel dash
fn handle_dashing(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Dashing, &mut GravityScale, &mut LinearVelocity)>,
) {
    for (entity, mut dashing, mut gravity_scale, mut linear_velocity) in &mut query {
        dashing.0.tick(time.delta());

        if dashing.0.finished() {
            commands
                .entity(entity)
                .remove::<Dashing>()
                .remove::<Flying>();
            gravity_scale.0 = CHARACTER_GRAVITY_SCALE;
            linear_velocity.x *= 0.4;
        }
    }
}
