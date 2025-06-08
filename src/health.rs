use std::collections::HashMap;

use avian2d::{
    math::AdjustPrecision,
    prelude::{Collider, CollidingEntities, CollisionLayers, Sensor},
};
use bevy::{prelude::*, transform};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Health>();
    app.add_event::<DeathEvent>()
        .add_event::<ChangeHpEvent>()
        .add_systems(
            Update,
            ((tick_hit_boxes, tick_hurt_boxes), get_hurt, change_hp).chain(),
        );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub const fn new(max: f32) -> Self {
        Self { max, current: max }
    }
}

#[derive(Event, Debug)]
pub struct DeathEvent(pub Entity);

#[derive(Event, Debug)]
pub struct ChangeHpEvent {
    target: Entity,
    amount: f32,
}

pub fn hurtbox_prefab(
    collider: Collider,
    collision_layer: CollisionLayers,
    full_rehit_delay: f32,
    transform: Transform,
) -> impl Bundle {
    (
        collider,
        collision_layer,
        CollidingEntities::default(),
        transform,
        Sensor,
        HurtBox {
            full_rehit_delay,
            remaining_rehit_delay: full_rehit_delay,
        },
    )
}

pub fn hitbox_prefab(
    collider: Collider,
    collision_layer: CollisionLayers,
    full_immunity_duration: f32,
    damage: f32,
    transform: Transform,
) -> impl Bundle {
    (
        collider,
        collision_layer,
        transform,
        Sensor,
        HitBox {
            full_immunity_duration,
            damage,
            remaining_immunity_duration: full_immunity_duration,
        },
    )
}

//This component should be ont he child of an entity with a health component
#[derive(Component)]
pub struct HurtBox {
    full_rehit_delay: f32,
    remaining_rehit_delay: f32,
}

//This component should be ont he child of an entity with a health component
#[derive(Component)]
pub struct HitBox {
    remaining_immunity_duration: f32,
    full_immunity_duration: f32,
    damage: f32,
}

fn tick_hurt_boxes(query: Query<&mut HurtBox>, time: Res<Time>) {
    for mut hb in query {
        hb.remaining_rehit_delay -= time.delta_secs_f64().adjust_precision();
    }
}
fn tick_hit_boxes(query: Query<&mut HitBox>, time: Res<Time>) {
    for mut hb in query {
        hb.remaining_immunity_duration -= time.delta_secs_f64().adjust_precision();
    }
}

fn get_hurt(
    mut query: Query<(Entity, &CollidingEntities, &mut HurtBox)>,
    mut hitboxes: Query<&mut HitBox>,
    mut hurt_event_writer: EventWriter<ChangeHpEvent>,
    parent_query: Query<&ChildOf>,
) {
    for (entity, colliding_entities, mut hurt_box) in &mut query {
        if hurt_box.remaining_rehit_delay > 0.0 {
            continue;
        }
        for hitbox_ent in colliding_entities.0.iter() {
            if let Ok(mut hb) = hitboxes.get_mut(*hitbox_ent) {
                if hb.remaining_immunity_duration <= 0.0 {
                    if let Ok(parent) = parent_query.get(entity) {
                        hurt_event_writer.write(ChangeHpEvent {
                            target: parent.parent(),
                            amount: -hb.damage,
                        });
                        hurt_box.remaining_rehit_delay = hurt_box.full_rehit_delay;
                        hb.remaining_immunity_duration = hb.full_immunity_duration;
                        break;
                    }
                }
            }
        }
    }
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
            health.current = (health.current + delta).min(health.max);
            if health.current <= 0.0 {
                death_event_writer.write(DeathEvent(entity));
            }
        }
    }
}
