use crate::physics::math;
use bevy::math::Vec3;
use std::f32::EPSILON;
use test_case::test_case;

#[derive(Debug)]
struct SphereAabbIntersectsCase {
    sphere_position: Vec3,
    radius: f32,
    aabb_position: Vec3,
    aabb_half_extents: Vec3,
    expected: bool,
}

#[derive(Debug)]
struct ClosestPointCase {
    point: Vec3,
    aabb_center: Vec3,
    half_extents: Vec3,
    expected: Vec3,
}

#[derive(Debug)]
struct ContactNormalCase {
    sphere_position: Vec3,
    sphere_radius: f32,
    aabb_position: Vec3,
    aabb_half_extents: Vec3,
    expected_normal: Vec3,
}

#[test_case(
    SphereAabbIntersectsCase {
        sphere_position: Vec3::ZERO,
        radius: 1.0,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(0.5, 0.5, 0.5),
        expected: true,
    };
    "sphere fully intersects"
)]
#[test_case(
    SphereAabbIntersectsCase {
        sphere_position: Vec3::new(2.0, 0.0, 0.0),
        radius: 1.0,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(0.5, 0.5, 0.5),
        expected: false,
    };
    "sphere far away"
)]
fn test_sphere_aabb_intersects(case: SphereAabbIntersectsCase) {
    let result = math::sphere_aabb_intersects(
        case.sphere_position,
        case.radius,
        case.aabb_position,
        case.aabb_half_extents,
    );
    assert_eq!(result, case.expected);
}

#[test_case(
    ClosestPointCase {
        point: Vec3::new(2.0, 0.0, 0.0),
        aabb_center: Vec3::ZERO,
        half_extents: Vec3::new(1.0, 1.0, 1.0),
        expected: Vec3::new(1.0, 0.0, 0.0),
    };
    "point outside x max"
)]
#[test_case(
    ClosestPointCase {
        point: Vec3::new(0.5, 0.5, 0.5),
        aabb_center: Vec3::ZERO,
        half_extents: Vec3::new(1.0, 1.0, 1.0),
        expected: Vec3::new(0.5, 0.5, 0.5),
    };
    "point inside aabb"
)]
fn test_closest_point_on_aabb(case: ClosestPointCase) {
    let result = math::closest_point_on_aabb(case.point, case.aabb_center, case.half_extents);
    assert!(
        (result - case.expected).length() < EPSILON,
        "expected {:?}, got {:?}",
        case.expected,
        result
    );
}

#[test_case(
    ContactNormalCase {
        sphere_position: Vec3::new(2.0, 0.0, 0.0),
        sphere_radius: 0.5,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        expected_normal: Vec3::X,
    };
    "contact normal x"
)]
#[test_case(
    ContactNormalCase {
        sphere_position: Vec3::new(0.0, -2.0, 0.0),
        sphere_radius: 0.5,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        expected_normal: -Vec3::Y,
    };
    "contact normal y"
)]
#[test_case(
    ContactNormalCase {
        sphere_position: Vec3::new(0.0, 0.0, 2.0),
        sphere_radius: 0.5,
        aabb_position: Vec3::ZERO,
        aabb_half_extents: Vec3::new(1.0, 1.0, 1.0),
        expected_normal: Vec3::Z,
    };
    "contact normal z"
)]
fn test_sphere_aabb_contact_normal(case: ContactNormalCase) {
    let normal = math::sphere_aabb_contact_normal(
        case.sphere_position,
        case.sphere_radius,
        case.aabb_position,
        case.aabb_half_extents,
    );
    assert!(
        (normal - case.expected_normal).length() < EPSILON,
        "expected {:?}, got {:?}",
        case.expected_normal,
        normal
    );
}
