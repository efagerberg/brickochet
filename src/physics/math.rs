use bevy::math::{Vec2, Vec3};

/// Result of a swept sphere-vs-AABB test.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct SweepHit {
    /// Fraction of this frame's motion (0.0..=1.0) at which contact occurs.
    /// Clamped to 0.0 if the sphere was already overlapping the box at the
    /// start of the frame (see `t_hit` below).
    pub t: f32,
    pub normal: Vec3,
    /// World-space position of contact. Correct even when the box itself
    /// is moving (see step 3 below) — no further adjustment needed by
    /// callers.
    pub contact_point: Vec3,
}

/// Sweeps a sphere along `sphere_velocity * dt` and checks whether it hits
/// an AABB — which may itself be moving along `aabb_velocity * dt` — at any
/// point along that relative path, not just at frame-start or frame-end
/// positions.
///
/// # Strategy: three ideas stacked together
///
/// **1. Minkowski trick, turn "sphere vs box" into "point vs box".**
/// A sphere touches a box exactly when the sphere's *center* enters a copy
/// of the box inflated by the sphere's radius in every direction. So we
/// inflate once up front and then only ever have to reason about a single
/// moving point vs. a box for the rest of the function.
///
/// **2. Relative motion, turn "two moving things" into "one thing moving,
/// one thing still".**
/// Only *relative* motion determines whether and when two things touch —
/// this is the same idea as judging whether two trains will collide by
/// looking at the difference in their speeds, not each one's speed
/// relative to the ground. Subtracting the box's velocity from the
/// sphere's gives the sphere's motion *as seen from the box's own rest
/// frame*, in which the box is stationary by construction. That lets the
/// rest of the function pretend it's solving the simpler "static box"
/// problem — because, in this frame, it genuinely is one.
///
/// **3. Slab method, turn "point vs box" into three 1D interval problems.**
/// An AABB is the intersection of three infinite slabs (x in [lo,hi], y in
/// [lo,hi], z in [lo,hi]). For each axis, solve for the time interval
/// `[t1, t2]` during which the moving point is inside *that axis's* slab
/// alone. The point is inside the actual box only during the overlap of
/// all three intervals, so intersect them: take the max of the entry
/// times and the min of the exit times. Whichever axis contributed the
/// *final* (latest) entry time is the face that was actually hit, which is
/// what gives us the collision normal for free.
///
/// # Converting back to world space
///
/// Everything above is computed in the box's rest frame, where the box
/// never moves. `t` and `normal` are frame-independent (a fraction of time
/// and a direction don't change under a change of reference frame), so
/// they're reported as-is. `contact_point`, however, is a *position* —
/// computed in the previous step as "where the sphere is, in a frame
/// where the box stayed put." To report a true world-space position, the
/// box's own displacement during the hit fraction has to be added back:
/// `contact_point = relative_contact_point + aabb_velocity * dt * t_hit`.
/// Skipping this step is subtle to notice — everything still compiles and
/// mostly looks right — but it silently reports a contact point that
/// lags behind a moving box's true position at the moment of impact,
/// worse the faster the box moves.
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
/// frame's relative motion (t in [0, 1]).
pub fn sweep_sphere_aabb(
    sphere_position: Vec3,
    sphere_velocity: Vec3,
    dt: f32,
    sphere_radius: f32,
    aabb_position: Vec3,
    aabb_half_extents: Vec3,
    aabb_velocity: Vec3,
) -> Option<SweepHit> {
    // --- Step 1: Minkowski trick. Inflate the box, now reason about a point. ---
    let expanded_half = aabb_half_extents + Vec3::splat(sphere_radius);
    let min = aabb_position - expanded_half;
    let max = aabb_position + expanded_half;

    // --- Step 2: relative motion. Reframe so the box is stationary. ---
    let relative_velocity = sphere_velocity - aabb_velocity;
    let motion = relative_velocity * dt;

    // --- Step 3: slab method. Intersect three per-axis time intervals. ---
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
            // No relative motion on this axis: the point's position on
            // this axis is fixed for the whole frame (in the box's rest
            // frame), so it either sits inside this slab the entire time
            // or never at all. No time interval to compute, just a
            // pass/fail gate.
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

    // Contact point in the box's rest frame...
    let relative_contact_point = sphere_position + motion * t_hit;
    // ...converted back to world space by adding the box's own
    // displacement during the hit fraction (see doc comment above).
    let contact_point = relative_contact_point + aabb_velocity * dt * t_hit;

    // The axis that set the final t_enter is the face we hit; the normal
    // points backward along that axis relative to the direction of
    // *relative* travel — still correct in world space, since direction
    // doesn't change under a frame shift by a constant velocity.
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

/// Result of a swept sphere-vs-bounded-plane test. A bounded plane is a
/// flat rectangular patch with a fixed normal and no depth: unlike an
/// AABB, it has exactly one face, so there's no possibility of a side or
/// edge hit — the sphere either crosses the (radius-offset) plane within
/// the rectangle's bounds, or it doesn't hit at all.
///
/// # Strategy
///
/// Same relative-motion reframing as `sweep_sphere_aabb`: subtract the
/// plane's velocity from the sphere's so the plane can be treated as
/// stationary. Then, instead of three per-axis slab intervals, there's
/// only one meaningful axis — distance along the plane's normal. Solve
/// for the single time `t` at which that distance crosses ±radius (the
/// Minkowski-offset plane), then check whether the contact point at that
/// moment actually falls within the plane's bounded width/height. If it
/// doesn't, the sphere missed the paddle's edge — no hit, regardless of
/// what's happening along the normal axis.
///
/// Assumes `plane_normal` is a unit vector and that the plane doesn't
/// rotate during the sweep (a translating, non-rotating panel — a
/// paddle).
pub fn sweep_sphere_plane(
    sphere_position: Vec3,
    sphere_velocity: Vec3,
    dt: f32,
    sphere_radius: f32,
    plane_position: Vec3,
    plane_velocity: Vec3,
    plane_normal: Vec3,
    plane_half_extents: Vec2, // bounds along the two axes tangent to the normal
) -> Option<SweepHit> {
    let relative_velocity = sphere_velocity - plane_velocity;
    let motion = relative_velocity * dt;

    // Signed distance from the sphere's center to the plane, along the
    // normal, at the start of the frame.
    let start_offset = (sphere_position - plane_position).dot(plane_normal);
    let closing_speed = motion.dot(plane_normal);

    // Which side of the plane the sphere starts on determines which
    // offset surface (+radius or -radius) it needs to cross.
    let side = start_offset.signum();
    let side = if side == 0.0 { 1.0 } else { side }; // exactly on-plane: pick a side arbitrarily
    let target_offset = side * sphere_radius;

    let t_hit = if closing_speed.abs() < f32::EPSILON {
        // No relative motion along the normal: either already within
        // radius of the plane (touching for the whole frame — report an
        // immediate hit) or never touches at all this frame.
        if start_offset.abs() <= sphere_radius {
            0.0
        } else {
            return None;
        }
    } else {
        let t = (target_offset - start_offset) / closing_speed;
        if t < 0.0 {
            // Only an immediate hit if we're still closing on the plane
            // (i.e. genuinely penetrating), not if we're embedded but
            // moving apart after a previous reflection.
            if start_offset.abs() <= sphere_radius && closing_speed * side < 0.0 {
                0.0
            } else {
                return None;
            }
        } else if t > 1.0 {
            return None; // Doesn't reach the plane within this frame.
        } else {
            t
        }
    };

    // Where the sphere's center is at the moment of contact, in the
    // plane's rest frame.
    let relative_contact_point = sphere_position + motion * t_hit;

    // Bounds check: project onto the plane's local tangent axes and
    // confirm the contact falls within the paddle's actual width/height,
    // not just somewhere on the infinite plane.
    let offset_from_center = relative_contact_point - plane_position;
    // Assumes plane_normal is axis-aligned (e.g. Vec3::Z) so the tangent
    // axes are simply the other two world axes. If the paddle can ever
    // face a non-axis-aligned direction, this needs a proper tangent
    // basis instead.
    let tangent_a = offset_from_center.x;
    let tangent_b = offset_from_center.y;
    if tangent_a.abs() > plane_half_extents.x || tangent_b.abs() > plane_half_extents.y {
        return None; // Missed the paddle's actual bounds — flew past its edge.
    }

    let contact_point = relative_contact_point + plane_velocity * dt * t_hit;

    Some(SweepHit {
        t: t_hit,
        normal: plane_normal * side,
        contact_point,
    })
}
