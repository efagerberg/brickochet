use bevy::math::Vec3;

pub fn sphere_aabb_intersects(
    sphere_position: Vec3,
    radius: f32,
    aabb_position: Vec3,
    aabb_half_extents: Vec3,
) -> bool {
    let closest = closest_point_on_aabb(sphere_position, aabb_position, aabb_half_extents);
    sphere_position.distance_squared(closest) <= radius * radius
}

/// Computes an axis-aligned contact normal for a sphere vs AABB collision
/// using smallest penetration depth.
pub fn sphere_aabb_contact_normal(
    sphere_position: Vec3,
    sphere_radius: f32,
    aabb_position: Vec3,
    aabb_half_extents: Vec3,
) -> Vec3 {
    let delta = sphere_position - aabb_position;
    let abs_delta = delta.abs();

    let overlap_x = aabb_half_extents.x + sphere_radius - abs_delta.x;
    let overlap_y = aabb_half_extents.y + sphere_radius - abs_delta.y;
    let overlap_z = aabb_half_extents.z + sphere_radius - abs_delta.z;

    // Assumes intersection already confirmed
    if overlap_x <= overlap_y && overlap_x <= overlap_z {
        Vec3::new(delta.x.signum(), 0.0, 0.0)
    } else if overlap_y <= overlap_z {
        Vec3::new(0.0, delta.y.signum(), 0.0)
    } else {
        Vec3::new(0.0, 0.0, delta.z.signum())
    }
}

pub fn closest_point_on_aabb(point: Vec3, aabb_center: Vec3, half_extents: Vec3) -> Vec3 {
    let min = aabb_center - half_extents;
    let max = aabb_center + half_extents;

    point.clamp(min, max)
}
