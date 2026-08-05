use bevy::prelude::*;
use test_case::test_case;

use crate::{physics::math, test_utils};

#[derive(Clone, Default)]
struct SweepSphereAabbCase {
    sphere_position: Vec3,
    sphere_velocity: Vec3,
    dt: f32,
    sphere_radius: f32,
    aabb_position: Vec3,
    aabb_half_extents: Vec3,
    aabb_velocity: Vec3,
    expected: Option<math::SweepHit>,
}

fn assert_sweep_hit_approx_eq(actual: math::SweepHit, expected: math::SweepHit) {
    test_utils::assertions::assert_floats_approx_eq(
        [
            actual.t,
            actual.normal.x,
            actual.normal.y,
            actual.normal.z,
            actual.contact_point.x,
            actual.contact_point.y,
            actual.contact_point.z,
        ],
        [
            expected.t,
            expected.normal.x,
            expected.normal.y,
            expected.normal.z,
            expected.contact_point.x,
            expected.contact_point.y,
            expected.contact_point.z,
        ],
    );
}

#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::new(-5.0, 0.0, 0.0),
        sphere_velocity: Vec3::new(10.0, 0.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.5,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: Some(math::SweepHit {
            t: 0.35,
            normal: Vec3::new(-1.0, 0.0, 0.0),
            contact_point: Vec3::new(-1.5, 0.0, 0.0),
        }),
    }; "straight-line hit on the x face, single dominant axis"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::new(-2.0, -2.0, 0.0),
        sphere_velocity: Vec3::new(4.0, 3.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.0,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: Some(math::SweepHit {
            t: 1.0 / 3.0,
            normal: Vec3::new(0.0, -1.0, 0.0),
            contact_point: Vec3::new(-2.0 / 3.0, -1.0, 0.0),
        }),
    }; "diagonal hit where y-slab entry is later than x-slab entry, so y wins"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::new(-5.0, 5.0, 0.0),
        sphere_velocity: Vec3::new(10.0, 0.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.5,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: None,
    }; "no collision - path stays entirely outside the y-slab"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::new(5.0, 0.0, 0.0),
        sphere_velocity: Vec3::new(10.0, 0.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.5,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: None,
    }; "no collision - moving away, overlap interval is entirely before t=0"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::ZERO,
        sphere_velocity: Vec3::new(5.0, 0.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.1,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: Some(math::SweepHit {
            t: 0.0,
            normal: Vec3::new(-1.0, 0.0, 0.0),
            contact_point: Vec3::ZERO,
        }),
    }; "regression: already overlapping at frame start reports an immediate hit at t=0, not a wrong-axis hit"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::ZERO,
        sphere_velocity: Vec3::ZERO,
        dt: 1.0,
        sphere_radius: 0.1,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: None,
    }; "stationary sphere already inside the box has no directed entry, so no hit is reported"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::new(-10.0, 0.0, 0.0),
        sphere_velocity: Vec3::new(9.0, 0.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.0,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: Some(math::SweepHit {
            t: 1.0,
            normal: Vec3::new(-1.0, 0.0, 0.0),
            contact_point: Vec3::new(-1.0, 0.0, 0.0),
        }),
    }; "boundary: entry happens at exactly t=1.0, inclusive, still counts as a hit"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::new(-10.0, 0.0, 0.0),
        sphere_velocity: Vec3::new(8.9, 0.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.0,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: None,
    }; "boundary: entry happens just after t=1.0, so this frame reports no hit"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::new(-1.0, -6.0, 0.0),
        sphere_velocity: Vec3::new(10.0, 10.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.0,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::ZERO,
        expected: None,
    }; "no collision - x-slab and y-slab windows never overlap, mid-loop early-out"
)]
#[test_case(
    SweepSphereAabbCase {
        sphere_position: Vec3::new(-5.0, 0.0, 0.0),
        sphere_velocity: Vec3::new(5.0, 0.0, 0.0),
        dt: 1.0,
        sphere_radius: 0.5,
        aabb_position: Vec3::new(5.0, 0.0, 0.0),
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        aabb_velocity: Vec3::new(-5.0, 0.0, 0.0),
        expected: Some(math::SweepHit {
            t: 0.85,
            normal: Vec3::new(-1.0, 0.0, 0.0),
            contact_point: Vec3::new(-0.75, 0.0, 0.0),
        }),
    }; "Moving sphere and aabb collision"
)]
fn test_sweep_sphere_aabb(case: SweepSphereAabbCase) {
    let actual = math::sweep_sphere_aabb(
        case.sphere_position,
        case.sphere_velocity,
        case.dt,
        case.sphere_radius,
        case.aabb_position,
        case.aabb_half_extents,
        case.aabb_velocity,
    );

    match (actual, case.expected) {
        (Some(actual), Some(expected)) => assert_sweep_hit_approx_eq(actual, expected),
        (None, None) => {}
        (actual, expected) => panic!("expected {expected:?}, got {actual:?} (Some/None mismatch)"),
    }
}
