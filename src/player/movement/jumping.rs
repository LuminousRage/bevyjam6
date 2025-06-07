use std::time::Duration;

use avian2d::prelude::{GravityScale, LinearVelocity};
use bevy::prelude::*;

use crate::player::configs::CHARACTER_GRAVITY_SCALE;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_jump_end);
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Jumping {
    pub duration: Timer,
    pub cooldown: Timer,
}

impl Jumping {
    pub fn new(duration: u64) -> Jumping {
        Self {
            duration: Timer::new(Duration::from_millis(duration), TimerMode::Once),
            cooldown: Timer::new(Duration::from_millis(100), TimerMode::Once),
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
