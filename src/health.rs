
use bevy::prelude::*;


pub(super) fn plugin(app: &mut App) {
    app.add_event::<DeathEvent>().add_systems(Update, (kill_no_hp).chain());
}

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}


#[derive(Event)]
struct DeathEvent(Entity);


fn kill_no_hp(mut death_event_writer: EventWriter<DeathEvent>, mut query: Query<(Entity,&Health)>) {
    for (entity,hp) in &mut query {
        if hp.current <= 0.0 {
            death_event_writer.write(DeathEvent(entity));
        }
    }
}