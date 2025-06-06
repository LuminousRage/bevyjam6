use avian2d::{
    math::{AdjustPrecision, Scalar, Vector},
    prelude::*,
};
use bevy::{
    math::ops::{exp, ln},
    prelude::*,
};

use crate::collision_layers::GameLayer;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_grounded, apply_movement_damping))
        .register_type::<MovementDampingFactor>()
        .register_type::<MaxSlopeAngle>();
}

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// A marker component indicating that an entity ignores air resistance and being grounded.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Flying;

/// The damping factor used for slowing down movement.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementDampingFactor(Scalar);
/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MaxSlopeAngle(Scalar);
/// A bundle that contains creature physics.
#[derive(Bundle)]
pub struct CreaturePhysicsBundle {
    damping: MovementDampingFactor,
    max_slope_angle: MaxSlopeAngle,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
}
/// A bundle that contains creature physics.
impl CreaturePhysicsBundle {
    pub fn new(collider: Collider, scale: Vector, damping: f32, max_slope_angle: f32) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(scale * 0.99, 10);

        Self {
            damping: MovementDampingFactor(damping),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
            body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(10.0)
                .with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Ground)),
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}
/// Updates the [`Grounded`] status for entities that dont Fly
fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>), Without<Flying>>,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_to(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity), Without<Flying>>,
    time: Res<Time>,
) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= exp(-time.delta_secs_f64().adjust_precision() * damping_factor.0);
    }
}
