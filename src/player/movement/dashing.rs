use avian2d::{
    math::AdjustPrecision,
    prelude::{GravityScale, LinearVelocity},
};
use bevy::prelude::*;

use crate::{
    physics::creature::{Flying, Grounded},
    player::configs::CHARACTER_GRAVITY_SCALE,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_dashing);
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Dashing {
    pub current_duration: f32,
    pub current_cooldown: f32,
    pub used: bool,
}

impl Dashing {
    pub fn new() -> Dashing {
        Self {
            current_duration: 0.0,
            current_cooldown: 0.0,
            used: false,
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
