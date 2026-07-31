use bevy::math::Vec3;

/// Result of a swept sphere-vs-AABB test.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SweepHit {
    /// Fraction of this frame's motion (0.0..=1.0) at which contact occurs.
    pub t: f32,
    pub normal: Vec3,
    pub contact_point: Vec3,
}

/// Sweeps a sphere along `velocity * dt` and checks whether it hits a static
/// AABB anywhere along that path, not just at the end position.
///
/// Returns `None` if the sphere's path never touches the box within this
/// frame's motion (t in [0, 1]).
pub fn sweep_sphere_aabb(
    sphere_position: Vec3,
    sphere_velocity: Vec3,
    dt: f32,
    sphere_radius: f32,
    aabb_position: Vec3,
    aabb_half_extents: Vec3,
) -> Option<SweepHit> {
    // Minkowski trick: inflate the box by the sphere's radius, then sweep a
    // single point (the sphere's center) against the inflated box.
    let expanded_half = aabb_half_extents + Vec3::splat(sphere_radius);
    let min = aabb_position - expanded_half;
    let max = aabb_position + expanded_half;

    let motion = sphere_velocity * dt;

    let mut t_enter = f32::NEG_INFINITY;
    let mut t_exit = f32::INFINITY;
    let mut enter_axis = 0usize; // 0 = x, 1 = y, 2 = z

    for axis in 0..3 {
        let start = sphere_position[axis];
        let d = motion[axis];
        let lo = min[axis];
        let hi = max[axis];

        if d.abs() < f32::EPSILON {
            // Not moving on this axis: must already be within the slab for
            // any hit to be possible at all.
            if start < lo || start > hi {
                return None;
            }
            continue;
        }

        let inv_d = 1.0 / d;
        let mut t1 = (lo - start) * inv_d;
        let mut t2 = (hi - start) * inv_d;
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }

        if t1 > t_enter {
            t_enter = t1;
            enter_axis = axis;
        }
        t_exit = t_exit.min(t2);

        if t_enter > t_exit {
            return None;
        }
    }

    // Every axis was a no-motion axis that was already inside the slab —
    // no relative motion into the box at all, so there's nothing new to report.
    if t_enter.is_infinite() {
        return None;
    }

    // Not going to collide
    if t_enter > 1.0 || t_exit < 0.0 {
        return None;
    }

    // If we're already overlapping at the start of the frame, report it as
    // an immediate hit (t = 0) rather than projecting the entry time backwards.
    let t_hit = t_enter.max(0.0);

    let contact_point = sphere_position + motion * t_hit;

    // Face-hit normal: outward along whichever axis we entered through.
    let mut normal = Vec3::ZERO;
    normal[enter_axis] = -motion[enter_axis].signum();

    // No corner or edge correction to keep the feel right

    Some(SweepHit {
        t: t_hit,
        normal,
        contact_point,
    })
}
