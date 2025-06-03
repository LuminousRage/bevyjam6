use std::collections::HashMap;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<DeathEvent>()
        .add_event::<ChangeHpEvent>()
        .add_systems(Update, (change_hp).chain());
}

#[derive(Component)]
pub struct Health {
    current: f32,
    max: f32,
}
impl Health {
    pub fn new(max: f32) -> Health {
        Health { max, current: max }
    }
}

#[derive(Event)]
pub struct DeathEvent(Entity);

#[derive(Event)]
pub struct ChangeHpEvent {
    target: Entity,
    amount: f32,
}

fn change_hp(
    mut change_hp_reader: EventReader<ChangeHpEvent>,
    mut death_event_writer: EventWriter<DeathEvent>,
    mut query: Query<&mut Health>,
) {
    let mut accumulated_deltas: HashMap<Entity, f32> = HashMap::new();

    for event in change_hp_reader.read() {
        *accumulated_deltas.entry(event.target).or_insert(0.0) += event.amount;
    }

    for (entity, delta) in accumulated_deltas {
        if let Ok(mut health) = query.get_mut(entity) {
            health.current = (health.current + delta).max(health.max);
            if health.current <= 0.0 {
                death_event_writer.write(DeathEvent(entity));
            }
        }
    }
}
