use std::collections::HashMap;

use avian2d::{
    math::AdjustPrecision,
    prelude::{Collider, CollidingEntities, CollisionLayers, Sensor},
};
use bevy::{prelude::*, sprite::Anchor};

use crate::{
    PausableSystems,
    player::{attack::systems::WowTheWeaponHit, weapon::WeaponHitbox},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Health>();
    app.add_event::<DeathEvent>()
        .add_event::<ChangeHpEvent>()
        .add_systems(
            Update,
            (
                ((tick_hit_boxes, tick_hurt_boxes), get_hurt, change_hp).chain(),
                update_health_bar,
            )
                .in_set(PausableSystems),
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
            remaining_rehit_delays: HashMap::default(),
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
    remaining_rehit_delays: HashMap<Entity, f32>,
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
        for v in hb.remaining_rehit_delays.values_mut() {
            *v -= time.delta_secs_f64().adjust_precision();
        }
    }
}
fn tick_hit_boxes(query: Query<&mut HitBox>, time: Res<Time>) {
    for mut hb in query {
        hb.remaining_immunity_duration -= time.delta_secs_f64().adjust_precision();
    }
}

#[derive(Component)]
pub struct HealthBar;

pub fn health_bar(transform: Transform, size: Vec2) -> impl Bundle {
    let back_colour = Color::srgb(0.4, 0.4, 0.4);
    let front_colour = Color::srgb(1.0, 0., 0.);

    (
        Name::new("Health Bar"),
        transform,
        Visibility::default(),
        children![
            (
                Name::new("Back"),
                Sprite {
                    custom_size: Some(size),
                    color: back_colour,
                    anchor: Anchor::CenterLeft,
                    ..default()
                }
            ),
            (
                Name::new("Front"),
                HealthBar,
                Sprite {
                    custom_size: Some(size),
                    color: front_colour,
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
            )
        ],
    )
}

fn update_health_bar(
    health_havers: Query<&Health>,
    health_bars: Query<(&ChildOf, &mut Transform), With<HealthBar>>,
    other_parents: Query<&ChildOf, Without<HealthBar>>,
) {
    for (child, mut healthbar_transform) in health_bars {
        let parent_parent = other_parents.get(child.parent());
        let parent_health = health_havers.get(parent_parent.unwrap().parent());
        if let Ok(health) = parent_health {
            let ratio = health.current / health.max;
            healthbar_transform.scale.x = ratio;
        }
    }
}

fn get_hurt(
    mut hurt_entities: Query<(Entity, &CollidingEntities, &mut HurtBox)>,
    mut hitboxes: Query<(&mut HitBox, Has<WeaponHitbox>)>,
    mut hurt_event_writer: EventWriter<ChangeHpEvent>,
    mut wow_the_weapon_hit: EventWriter<WowTheWeaponHit>,
    parent_query: Query<&ChildOf>,
) {
    for (hurt_entity, hurt_box_colliding_entities, mut hurt_box) in &mut hurt_entities {
        if *hurt_box
            .remaining_rehit_delays
            .get(&hurt_entity)
            .unwrap_or(&-1.)
            > 0.0
        {
            continue;
        }
        for hitbox_ent in hurt_box_colliding_entities.0.iter() {
            match (hitboxes.get_mut(*hitbox_ent), parent_query.get(hurt_entity)) {
                (Ok((mut hitb, is_weapon_hitbox)), Ok(parent)) => {
                    if hitb.remaining_immunity_duration <= 0.0 {
                        hurt_event_writer.write(ChangeHpEvent {
                            target: parent.parent(),
                            amount: -hitb.damage,
                        });

                        if is_weapon_hitbox {
                            wow_the_weapon_hit.write(WowTheWeaponHit);
                        }
                        let v = hurt_box.full_rehit_delay;
                        hurt_box.remaining_rehit_delays.insert(hurt_entity, v);
                        hitb.remaining_immunity_duration = hitb.full_immunity_duration;
                        break;
                    }
                }
                _ => {}
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
