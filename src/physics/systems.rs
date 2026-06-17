use crate::physics;
use bevy::prelude::*;
use std::collections::HashSet;

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
    spheres: Query<(Entity, &Transform, &physics::components::BoundingSphere)>,
    cuboids: Query<(Entity, &Transform, &physics::components::BoundingCuboid)>,
    mut messages: MessageWriter<physics::messages::CollisionMessage>,
) {
    let mut processed_entities: HashSet<Entity> = HashSet::new(); // Track already collided entities

    for (a_entity, a_transform, a_bounds) in spheres.iter() {
        // Skip if this sphere has already collided with another entity
        if processed_entities.contains(&a_entity) {
            continue;
        }

        for (b_entity, b_transform, b_bounds) in cuboids.iter() {
            // Skip if this cuboid has already collided with another entity
            if processed_entities.contains(&b_entity) {
                continue;
            }

            // Check for intersection
            if physics::math::sphere_aabb_intersects(
                a_transform.translation,
                a_bounds.radius,
                b_transform.translation,
                b_bounds.half_extents,
            ) {
                let normal = physics::math::sphere_aabb_contact_normal(
                    a_transform.translation,
                    a_bounds.radius,
                    b_transform.translation,
                    b_bounds.half_extents,
                );

                let contact_point = physics::math::closest_point_on_aabb(
                    a_transform.translation,
                    b_transform.translation,
                    b_bounds.half_extents,
                );

                let penetration = a_bounds.radius - contact_point.distance(a_transform.translation);

                // Create a collision message
                messages.write(physics::messages::CollisionMessage {
                    a: a_entity,
                    b: b_entity,
                    normal,
                    contact_point,
                    penetration,
                });

                // Mark both entities as processed
                processed_entities.insert(a_entity);
                processed_entities.insert(b_entity);

                break; // Exit the inner loop after a collision to prevent further collisions in this frame
            }
        }
    }
}

pub fn resolve_sphere_aabb_collision(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        (&mut physics::components::Velocity, &mut Transform),
        With<physics::components::BoundingSphere>,
    >,
    _cuboid_query: Query<
        Entity,
        (
            With<physics::components::BoundingCuboid>,
            Without<physics::components::BoundingSphere>,
        ),
    >,
) {
    // Collect all collisions per sphere
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
            // Only consider collisions with valid cuboids
            for message in collisions {
                // Move the sphere out of the cuboid
                transform.translation += message.normal * message.penetration;

                // Reflect velocity once
                velocity.0 = velocity.0.reflect(message.normal);
            }
        }
    }
}
