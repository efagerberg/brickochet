use crate::physics;
use bevy::prelude::*;

pub fn apply_velocity(
    time: Res<Time>,
    query: Query<(&mut Transform, &physics::components::Velocity)>,
) {
    let delta_secs = time.delta_secs();
    for (mut transform, velocity) in query {
        transform.translation += velocity.0 * delta_secs;
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
    spheres: Query<(
        Entity,
        &Transform,
        &physics::components::BoundingSphere,
        &physics::components::Velocity,
    )>,
    cuboids: Query<(Entity, &Transform, &physics::components::BoundingCuboid)>,
    time: Res<Time>,
    mut messages: MessageWriter<physics::messages::CollisionMessage>,
) {
    for (a_entity, a_transform, a_bounds, a_velocity) in spheres.iter() {
        let mut earliest: Option<(Entity, physics::math::SweepHit)> = None;

        for (b_entity, b_transform, b_bounds) in cuboids.iter() {
            if let Some(hit) = physics::math::sweep_sphere_aabb(
                a_transform.translation,
                a_velocity.0,
                time.delta_secs(),
                a_bounds.radius,
                b_transform.translation,
                b_bounds.half_extents,
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
        if let Some((b_entity, hit)) = earliest {
            let contact_center = a_transform.translation + a_velocity.0 * time.delta_secs() * hit.t;
            messages.write(physics::messages::CollisionMessage {
                a: a_entity,
                b: b_entity,
                normal: hit.normal,
                contact_point: contact_center,
                time_of_impact: hit.t,
            });
        }
    }
}

pub fn resolve_sphere_aabb_collision(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        (&mut physics::components::Velocity, &mut Transform),
        With<physics::components::BoundingSphere>,
    >,
    time: Res<Time>,
) {
    let mut collisions_per_sphere: std::collections::HashMap<
        Entity,
        Vec<&physics::messages::CollisionMessage>,
    > = std::collections::HashMap::new();
    for message in messages.read() {
        collisions_per_sphere
            .entry(message.a)
            .or_default()
            .push(message);
    }

    for (sphere_entity, collisions) in collisions_per_sphere {
        if let Ok((mut velocity, mut transform)) = sphere_query.get_mut(sphere_entity) {
            for message in collisions {
                transform.translation = message.contact_point;
                // Only reflect the velocity if the ball is actually moving
                // *into* the surface. Without this check, a ball that's
                // already separating (e.g. after being resolved against a
                // different contact this same frame) gets its velocity
                // flipped right back toward the wall, which is what was
                // producing the stick-then-escape jitter.
                let approach_speed = velocity.0.dot(message.normal);
                if approach_speed < 0.0 {
                    velocity.0 = velocity.0.reflect(message.normal);
                }
                let remaining = (1.0 - message.time_of_impact) * time.delta_secs();
                transform.translation =
                    message.contact_point + message.normal * 1e-4 + velocity.0 * remaining;
            }
        }
    }
}
