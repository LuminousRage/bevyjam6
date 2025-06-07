use avian2d::{
    math::AdjustPrecision,
    prelude::{GravityScale, LinearVelocity},
};
use bevy::prelude::*;

use crate::{
    physics::creature::{Flying, Grounded},
    player::{
        character::Player,
        configs::{
            CHARACTER_GRAVITY_SCALE, DASH_COOLDOWN_DURATION, DASH_DURATION, DASH_SPEED_MODIFIER,
            JUMP_DURATION_SECONDS, MOVEMENT_SPEED,
        },
        movement::movement::PlayerMovementState,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<DashingEvent>();
    app.add_systems(Update, (handle_dashing, handle_dashing_cooldown));
}

#[derive(Event)]
pub struct DashingEvent {
    /// false cancels the dash
    pub is_start: bool,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DashingUsed;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DashingCooldown(Timer);

pub fn handle_dash_event(
    player: Single<(
        Entity,
        &Player,
        &mut LinearVelocity,
        &mut GravityScale,
        &mut PlayerMovementState,
        Has<DashingUsed>,
        Has<DashingCooldown>,
    )>,
    mut dash_event_reader: EventReader<DashingEvent>,
    mut commands: Commands,
) {
    let (
        entity,
        player,
        mut linear_velocity,
        mut gravity,
        mut movement_state,
        used_dashing,
        dashing_cooldown,
    ) = player.into_inner();

    for event in dash_event_reader.read() {
        match event.is_start {
            true => {
                if let PlayerMovementState::Dash(_) = *movement_state {
                    continue;
                }
                if used_dashing || dashing_cooldown {
                    // Can't use dash, do nothing
                    continue;
                }
                commands.entity(entity).insert(Flying);
                linear_velocity.x = player.face_direction.x * MOVEMENT_SPEED * DASH_SPEED_MODIFIER;
                linear_velocity.y = 0.0;
                gravity.0 = 0.0;
                commands.entity(entity).insert(DashingUsed);
                *movement_state = PlayerMovementState::Dash(DASH_DURATION);
            }
            false => {
                commands.entity(entity).remove::<Flying>();
                gravity.0 = CHARACTER_GRAVITY_SCALE;
                linear_velocity.x *= 0.4;
                *movement_state = PlayerMovementState::Jump(Timer::from_seconds(
                    JUMP_DURATION_SECONDS,
                    TimerMode::Once,
                ));
                commands
                    .entity(entity)
                    .insert(DashingCooldown(Timer::from_seconds(
                        DASH_COOLDOWN_DURATION,
                        TimerMode::Once,
                    )));
            }
        }
    }
}

fn handle_dashing_cooldown(
    mut commands: Commands,
    player: Option<Single<(Entity, &mut DashingCooldown), With<Player>>>,
    time: Res<Time>,
) {
    if let Some(p) = player {
        let (entity, mut cooldown) = p.into_inner();
        cooldown.0.tick(time.delta());
        if cooldown.0.finished() {
            commands.entity(entity).remove::<DashingCooldown>();
        }
    }
}

fn handle_dashing(
    time: Res<Time>,
    mut commands: Commands,
    mut dash_event_writer: EventWriter<DashingEvent>,
    player: Single<(Entity, &mut PlayerMovementState, Has<Grounded>), With<Player>>,
) {
    let (entity, mut state, is_grounded) = player.into_inner();

    if let PlayerMovementState::Dash(duration) = &mut *state {
        let delta = time.delta().as_secs_f32().adjust_precision();
        *duration -= delta;
        if *duration <= 0.0 && *duration + delta > 0.0 {
            dash_event_writer.write(DashingEvent { is_start: false });
        }
    }

    if is_grounded {
        commands.entity(entity).remove::<DashingUsed>();
    }
}
