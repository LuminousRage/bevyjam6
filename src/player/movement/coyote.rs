use std::time::Duration;

use bevy::prelude::*;

use crate::{physics::creature::Grounded, player::character::Player};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Coyote(Timer);

impl Coyote {
    fn new(duration: u64) -> Coyote {
        Self(Timer::new(Duration::from_millis(duration), TimerMode::Once))
    }
}

pub fn detect_coyote_time_start(
    entity: Single<Entity, (With<Player>, With<Grounded>, Without<Coyote>)>,
    mut commands: Commands,
) {
    commands.entity(*entity).insert(Coyote::new(200));
}

pub fn handle_coyote_time(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Coyote), (Without<Grounded>, With<Player>)>,
) {
    for (entity, mut coyote) in &mut query {
        coyote.0.tick(time.delta());

        if coyote.0.finished() {
            commands.entity(entity).remove::<Coyote>();
        }
    }
}
