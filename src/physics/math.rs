use bevy::math::Vec3;

pub fn sphere_aabb_intersects(
    sphere_pos: Vec3,
    radius: f32,
    box_pos: Vec3,
    box_half: Vec3,
) -> bool {
    let closest = sphere_pos.clamp(box_pos - box_half, box_pos + box_half);

    sphere_pos.distance_squared(closest) <= radius * radius
}
