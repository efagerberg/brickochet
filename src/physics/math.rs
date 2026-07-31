use bevy::math::Vec3;

/// Result of a swept sphere-vs-AABB test.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SweepHit {
    /// Fraction of this frame's motion (0.0..=1.0) at which contact occurs.
    /// Clamped to 0.0 if the sphere was already overlapping the box at the
    /// start of the frame (see `t_hit` below).
    pub t: f32,
    pub normal: Vec3,
    pub contact_point: Vec3,
}

/// Sweeps a sphere along `velocity * dt` and checks whether it hits a static
/// AABB anywhere along that path, not just at the end position.
///
/// # Strategy: two ideas stacked together
///
/// **1. Minkowski trick, turn "sphere vs box" into "point vs box".**
/// A sphere touches a box exactly when the sphere's *center* enters a copy
/// of the box inflated by the sphere's radius in every direction. So we
/// inflate once up front and then only ever have to reason about a single
/// moving point vs. a static box for the rest of the function.
///
/// **2. Slab method, turn "point vs box" into three 1D interval problems.**
/// An AABB is the intersection of three infinite slabs (x in [lo,hi], y in
/// [lo,hi], z in [lo,hi]). For each axis, solve for the time interval
/// `[t1, t2]` during which the moving point is inside *that axis's* slab
/// alone. The point is inside the actual box only during the overlap of
/// all three intervals, so intersect them: take the max of the entry
/// times and the min of the exit times. Whichever axis contributed the
/// *final* (latest) entry time is the face that was actually hit, which is
/// what gives us the collision normal for free.
///
/// # Why the sentinels are +/-infinity, not 0.0/1.0
///
/// `t_enter`/`t_exit` start as the *universal* interval (-inf, +inf) and
/// get narrowed down axis by axis via max/min, that's the correct
/// identity for an interval intersection. Seeding `t_enter` at `0.0`
/// instead is a subtle bug: it silently assumes the sphere can never have
/// "entered" before this frame started. That assumption breaks the moment
/// the sphere begins the frame already overlapping the box on the winning
/// axis (e.g. right after a previous collision resolved with only a tiny
/// push-out margin), the true entry time is negative, `t1 > 0.0` never
/// fires, and `enter_axis` silently stays at its default instead of
/// updating to the axis that's actually responsible. You get a hit with
/// the wrong normal, which downstream code will use to reflect velocity
/// incorrectly (or not at all).
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
    // --- Step 1: Minkowski trick. Inflate the box, now reason about a point. ---
    let expanded_half = aabb_half_extents + Vec3::splat(sphere_radius);
    let min = aabb_position - expanded_half;
    let max = aabb_position + expanded_half;

    let motion = sphere_velocity * dt;

    // --- Step 2: slab method. Intersect three per-axis time intervals. ---
    // Start as the universal interval; each axis can only shrink it.
    let mut t_enter = f32::NEG_INFINITY;
    let mut t_exit = f32::INFINITY;
    let mut enter_axis = 0usize; // 0 = x, 1 = y, 2 = z, the axis that set t_enter

    for axis in 0..3 {
        let start = sphere_position[axis];
        let d = motion[axis];
        let lo = min[axis];
        let hi = max[axis];

        if d.abs() < f32::EPSILON {
            // No motion on this axis: the point's position on this axis is
            // fixed for the whole frame, so it either sits inside this
            // slab the entire time or never at all. No time interval to
            // compute, just a pass/fail gate.
            if start < lo || start > hi {
                return None;
            }
            continue;
        }

        // Solve start + d*t = lo and start + d*t = hi for t. Sort so t1 is
        // always "enters this slab" and t2 is "exits this slab", regardless
        // of which direction the point is moving.
        let inv_d = 1.0 / d;
        let mut t1 = (lo - start) * inv_d;
        let mut t2 = (hi - start) * inv_d;
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }

        // Intersect this axis's interval into the running result: the
        // point isn't inside *every* slab until the last one it enters,
        // and it stops being inside the box the moment it leaves *any*
        // one slab.
        if t1 > t_enter {
            t_enter = t1;
            enter_axis = axis;
        }
        t_exit = t_exit.min(t2);

        // Early out: if the latest entry is already after the earliest
        // exit, the slabs never overlap in time at all.
        if t_enter > t_exit {
            return None;
        }
    }

    // Every axis was a no-motion axis that happened to already be inside
    // its slab: there's no directed entry into the box at all (the point
    // is either fully static relative to the box, or this is a degenerate
    // all-axes-stationary case). Nothing meaningful to report.
    if t_enter.is_infinite() {
        return None;
    }

    // The collision (if any) happens outside this frame's motion window.
    if t_enter > 1.0 || t_exit < 0.0 {
        return None;
    }

    // If the sphere was already overlapping the box at the start of the
    // frame, t_enter will be negative, the "true" entry time is in the
    // past. We can't un-penetrate retroactively, so report the hit as
    // happening immediately (t = 0) instead. This is the value that
    // should drive anything time-dependent about the hit, including the
    // contact point below.
    let t_hit = t_enter.max(0.0);
    let contact_point = sphere_position + motion * t_hit;

    // The axis that set the final t_enter is the face we hit; the normal
    // points backward along that axis relative to the direction of travel.
    let mut normal = Vec3::ZERO;
    normal[enter_axis] = -motion[enter_axis].signum();

    // Deliberately no corner/edge rounding of the normal, single-face
    // normals only, to keep the bounce feel predictable.

    Some(SweepHit {
        t: t_hit,
        normal,
        contact_point,
    })
}
