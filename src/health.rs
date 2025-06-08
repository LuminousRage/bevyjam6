use std::collections::HashMap;

use avian2d::{
    math::AdjustPrecision,
    prelude::{
        Collider, CollidingEntities, CollisionEventsEnabled, CollisionLayers, CollisionStarted,
        Sensor,
    },
};
use bevy::{prelude::*, sprite::Anchor};

use crate::player::{attack::systems::WowTheWeaponHit, character::Player, weapon::WeaponHitbox};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Health>();
    app.add_event::<DeathEvent>()
        .add_event::<ChangeHpEvent>()
        .add_systems(
            Update,
            (
                ((tick_hit_boxes, tick_hurt_boxes), get_hurt, change_hp).chain(),
                update_health_bar,
            ),
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
        CollisionEventsEnabled,
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
        CollisionEventsEnabled,
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
    mut hurt_entities: Query<(Entity, &mut HurtBox, Has<Player>)>,
    mut hitboxes: Query<(Entity, &mut HitBox, Has<WeaponHitbox>)>,
    mut hurt_event_writer: EventWriter<ChangeHpEvent>,
    mut wow_the_weapon_hit: EventWriter<WowTheWeaponHit>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    parent_query: Query<&ChildOf>,
) {
    for e in collision_event_reader.read() {
        let hurtbox = hurt_entities.get(e.0).or(hurt_entities.get(e.1));
        let hitbox = hitboxes.get(e.0).or(hitboxes.get(e.1));

        let (hurtbox, hitbox) = match (hurtbox, hitbox) {
            (Ok(hurt), Ok(hit)) => (
                hurt_entities.get_mut(hurt.0).unwrap(),
                hitboxes.get_mut(hit.0).unwrap(),
            ),
            _ => {
                continue;
            }
        };
        let (entity, hurtbox, has_player) = hurtbox;
        let (_, mut hitbox, is_weapon_hitbox) = hitbox;
        if !has_player && !is_weapon_hitbox || has_player && is_weapon_hitbox {
            continue;
        }
        if hurtbox.remaining_rehit_delay > 0.0 {
            continue;
        }
        let hurt_entity = parent_query.get(entity).unwrap();

        if hitbox.remaining_immunity_duration <= 0.0 {
            hurt_event_writer.write(ChangeHpEvent {
                target: hurt_entity.parent(),
                amount: -hitbox.damage,
            });

            if is_weapon_hitbox {
                wow_the_weapon_hit.write(WowTheWeaponHit);
            }

            hitbox.remaining_immunity_duration = hitbox.full_immunity_duration;
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
