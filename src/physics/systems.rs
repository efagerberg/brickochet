use crate::physics;
use bevy::{ecs::entity, prelude::*};

/// Minimum gap enforced between a sphere and a surface it just collided
/// with, applied along the collision normal after resolving contact.
///
/// This exists so the *next* frame's sweep (`sweep_sphere_aabb`) starts
/// from a position that is unambiguously outside the surface rather than
/// exactly on it or numerically inside it — without this, float error can
/// put the sphere a hair on the wrong side of the boundary, which the
/// sweep's "already overlapping" path (t_enter <= 0) then has to recover
/// from every frame.
///
/// Chosen relative to world units, not sphere radius or velocity — if
/// world units are small (sub-1.0 scale) or velocities are very high,
/// this may need to grow; too small and it stops being effective against
/// float noise, too large and it becomes a visible pop at contact.
pub const COLLISION_SEPARATION_EPSILON: f32 = 1e-4;

pub fn apply_velocity(
    time: Res<Time>,
    query: Query<(Entity, &mut Transform, &physics::components::Velocity)>,
    mut messages: MessageReader<physics::messages::CollisionMessage>,
) {
    // Invariant: `detect_collisions` writes at most one message per sphere
    // entity (it already picks the earliest hit). If that ever changes,
    // this map will silently keep only the last message written for a
    // given entity, worth revisiting this data structure if you add
    // multi-contact resolution per frame.
    let mut entity_to_message: entity::EntityHashMap<&physics::messages::CollisionMessage> =
        entity::EntityHashMap::new();
    for message in messages.read() {
        entity_to_message.insert(message.a, message);
    }
    let delta_secs = time.delta_secs();
    for (entity, mut transform, velocity) in query {
        if let Some(message) = entity_to_message.remove(&entity) {
            let remaining_secs = (1.0 - message.hit.t) * delta_secs;
            transform.translation = message.hit.contact_point
                + message.hit.normal * COLLISION_SEPARATION_EPSILON
                + velocity.0 * remaining_secs;
        } else {
            transform.translation += velocity.0 * delta_secs;
        }
    }
}

pub fn apply_curve_spin(
    time: Res<Time>,
    query: Query<(&physics::components::Curve, &mut Transform)>,
) {
    let delta_secs = time.delta_secs();
    for (curve, mut transform) in query {
        // Spin axis is perpendicular to the curve direction
        // e.g. curving left/right = spinning around Z, curving up/down = spinning around X
        let spin_axis = Vec3::new(curve.0.y, -curve.0.x, 0.0).normalize_or_zero();
        let spin_rate = curve.0.length(); // stronger curve = faster spin

        if spin_rate > 0.0 {
            transform.rotate(Quat::from_axis_angle(spin_axis, spin_rate * delta_secs));
        }
    }
}

pub fn add_curve_velocity(
    time: Res<Time>,
    query: Query<(
        &mut physics::components::Velocity,
        &physics::components::Curve,
    )>,
) {
    let delta_secs = time.delta_secs();
    for (mut velocity, curve) in query {
        velocity.0.x += curve.0.x * delta_secs;
        velocity.0.y += curve.0.y * delta_secs;
    }
}

pub fn detect_collisions(
    spheres: Query<
        (
            Entity,
            &Transform,
            &physics::components::SphereCollider,
            &physics::components::Velocity,
        ),
        With<physics::components::DynamicBody>,
    >,
    cuboids: Query<(
        Entity,
        &Transform,
        &physics::components::CuboidCollider,
        Option<&physics::components::Velocity>,
    )>,
    time: Res<Time>,
    mut messages: MessageWriter<physics::messages::CollisionMessage>,
) {
    for (a_entity, a_transform, a_collider, a_velocity) in spheres.iter() {
        let mut earliest: Option<(Entity, physics::math::SweepHit)> = None;

        for (b_entity, b_transform, b_collider, b_velocity) in cuboids.iter() {
            if let Some(hit) = physics::math::sweep_sphere_aabb(
                a_transform.translation,
                a_velocity.0,
                time.delta_secs(),
                a_collider.radius,
                b_transform.translation,
                b_collider.half_extents,
                b_velocity.map(|v| v.0).unwrap_or(Vec3::ZERO),
            ) {
                let is_earlier = earliest
                    .as_ref()
                    .map(|(_, prev)| hit.t < prev.t)
                    .unwrap_or(true);
                if is_earlier {
                    earliest = Some((b_entity, hit));
                }
            }
        }

        // Take earliest collision event
        if let Some((b_entity, hit)) = earliest {
            messages.write(physics::messages::CollisionMessage {
                a: a_entity,
                b: b_entity,
                hit,
            });
        }
    }
}

pub fn resolve_sphere_aabb_collision(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        &mut physics::components::Velocity,
        (
            With<physics::components::SphereCollider>,
            With<physics::components::DynamicBody>,
        ),
    >,
) {
    let mut collisions_per_sphere: entity::EntityHashMap<
        Vec<&physics::messages::CollisionMessage>,
    > = entity::EntityHashMap::new();
    for message in messages.read() {
        collisions_per_sphere
            .entry(message.a)
            .or_default()
            .push(message);
    }

    for (sphere_entity, collisions) in collisions_per_sphere {
        if let Ok(mut velocity) = sphere_query.get_mut(sphere_entity) {
            for message in collisions {
                // Only reflect the velocity if the ball is actually moving
                // *into* the surface. Without this check, a ball that's
                // already separating (e.g. after being resolved against a
                // different contact this same frame) gets its velocity
                // flipped right back toward the wall, which is what was
                // producing the stick-then-escape jitter.
                let approach_speed = velocity.0.dot(message.hit.normal);
                if approach_speed < 0.0 {
                    velocity.0 = velocity.0.reflect(message.hit.normal);
                }
            }
        }
    }
}
