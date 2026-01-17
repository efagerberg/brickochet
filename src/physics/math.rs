use bevy::math::Vec3;

pub fn sphere_aabb_intersects(
    sphere_pos: Vec3,
    radius: f32,
    box_pos: Vec3,
    box_half: Vec3,
) -> bool {
    let closest = sphere_pos.clamp(
        box_pos - box_half,
        box_pos + box_half,
    );

    sphere_pos.distance_squared(closest) <= radius * radius
}

pub fn sphere_aabb_normal(
    sphere_pos: Vec3,
    box_pos: Vec3,
    box_half: Vec3,
) -> Vec3 {
    let delta = sphere_pos - box_pos;
    let clamped = delta.clamp(-box_half, box_half);
    let closest = box_pos + clamped;

    (sphere_pos - closest).normalize_or_zero()
}
