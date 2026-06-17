use bevy::prelude::*;
use test_case::test_case;

use crate::physics;

#[derive(Default)]
struct ApplyVelocityCase {
    velocity: Vec3,
    delta_secs: f32,
    expected_translation: Vec3,
}

#[test_case(
    ApplyVelocityCase {
        velocity: Vec3::new(1.0, 0.0, 0.0),
        delta_secs: 1.0,
        expected_translation: Vec3::new(1.0, 0.0, 0.0),
    }
; "moves 1 unit in x over 1 second")]
#[test_case(
    ApplyVelocityCase {
        velocity: Vec3::new(0.0, 2.0, 0.0),
        delta_secs: 0.5,
        expected_translation: Vec3::new(0.0, 1.0, 0.0),
    }
; "moves half velocity over half second")]
fn test_apply_velocity_moves_transform(case: ApplyVelocityCase) {
    let mut app = App::new();

    let entity = app
        .world_mut()
        .spawn((
            Transform::default(),
            physics::components::Velocity(case.velocity),
        ))
        .id();

    let mut time: Time = Time::default();
    time.advance_by(std::time::Duration::from_secs_f32(case.delta_secs));
    app.insert_resource(time);
    app.add_systems(Update, physics::systems::apply_velocity);

    app.update();

    let transform = app.world().get::<Transform>(entity).unwrap();
    assert_eq!(transform.translation, case.expected_translation);
}

#[derive(Default)]
struct AddCurveVelocityCase {
    initial_velocity: Vec3,
    curve: Vec2,
    delta_secs: f32,
    expected_velocity: Vec3,
}

#[test_case(
    AddCurveVelocityCase {
        initial_velocity: Vec3::new(1.0, 1.0, 0.0),
        curve: Vec2::new(0.5, -0.25),
        delta_secs: 1.0,
        expected_velocity: Vec3::new(1.5, 0.75, 0.0),
    }
; "curve modifies x and y velocity")]
#[test_case(
    AddCurveVelocityCase {
        initial_velocity: Vec3::ZERO,
        curve: Vec2::new(1.0, 1.0),
        delta_secs: 1.0,
        expected_velocity: Vec3::new(1.0, 1.0, 0.0),
    }
; "curve applied to zero velocity")]
fn test_add_curve_velocity_modifies_velocity(case: AddCurveVelocityCase) {
    let mut app = App::new();

    let entity = app
        .world_mut()
        .spawn((
            physics::components::Velocity(case.initial_velocity),
            physics::components::Curve(case.curve),
        ))
        .id();

    app.add_systems(Update, physics::systems::add_curve_velocity);

    let mut time: Time = Time::default();
    time.advance_by(std::time::Duration::from_secs_f32(case.delta_secs));
    app.insert_resource(time);

    app.update();

    let velocity = app
        .world()
        .get::<physics::components::Velocity>(entity)
        .unwrap();
    assert_eq!(velocity.0, case.expected_velocity);
}

struct ApplyCurveSpinCase {
    curve: Vec2,
    delta_secs: f32,
    expected_axis: Vec3,
    expected_angle_rads: f32,
}

#[test_case(ApplyCurveSpinCase {
    curve: Vec2::new(1.0, 0.0),
    delta_secs: 1.0,
    expected_axis: Vec3::NEG_Y,
    expected_angle_rads: 1.0,
} ; "positive X curve spins around NEG_Y")]
#[test_case(ApplyCurveSpinCase {
    curve: Vec2::new(0.0, 1.0),
    delta_secs: 1.0,
    expected_axis: Vec3::X,
    expected_angle_rads: 1.0,
} ; "positive Y curve spins around X")]
#[test_case(ApplyCurveSpinCase {
    curve: Vec2::new(-1.0, 0.0),
    delta_secs: 1.0,
    expected_axis: Vec3::Y,
    expected_angle_rads: 1.0,
} ; "negative X curve spins around Y")]
#[test_case(ApplyCurveSpinCase {
    curve: Vec2::new(0.0, -1.0),
    delta_secs: 1.0,
    expected_axis: Vec3::NEG_X,
    expected_angle_rads: 1.0,
} ; "negative Y curve spins around NEG_X")]
#[test_case(ApplyCurveSpinCase {
    curve: Vec2::new(1.0, 0.0),
    delta_secs: 2.0,
    expected_axis: Vec3::NEG_Y,
    expected_angle_rads: 2.0,
} ; "rotation scales with delta_secs")]
#[test_case(ApplyCurveSpinCase {
    curve: Vec2::ZERO,
    delta_secs: 1.0,
    expected_axis: Vec3::ZERO,
    expected_angle_rads: 0.0,
} ; "zero curve produces zero rotation")]
fn test_apply_curve_spin_adds_spin_to_transform(case: ApplyCurveSpinCase) {
    let mut app = App::new();

    let entity = app
        .world_mut()
        .spawn((
            physics::components::Curve(case.curve),
            Transform::default()
        ))
        .id();

    app.add_systems(Update, physics::systems::apply_curve_spin);

    let mut time: Time = Time::default();
    time.advance_by(std::time::Duration::from_secs_f32(case.delta_secs));
    app.insert_resource(time);

    app.update();

    let transform = app
        .world()
        .get::<Transform>(entity)
        .unwrap();
        let expected = Quat::from_axis_angle(case.expected_axis, case.expected_angle_rads);
        let actual = transform.rotation;

        assert_eq!(expected, actual);
}

struct DetectCollisionCase {
    sphere_translation: Vec3,
    sphere_radius: f32,
    cuboid_translation: Vec3,
    cuboid_half_extents: Vec3,
    should_collide: bool,
}

#[test_case(
    DetectCollisionCase {
        sphere_translation: Vec3::new(0.0, 0.0, 0.0),
        sphere_radius: 1.0,
        cuboid_translation: Vec3::new(1.5, 0.0, 0.0),
        cuboid_half_extents: Vec3::new(1.0, 1.0, 1.0),
        should_collide: true,
    }; "sphere collides with cuboid")]
#[test_case(
    DetectCollisionCase {
        sphere_translation: Vec3::new(0.0, 0.0, 0.0),
        sphere_radius: 1.0,
        cuboid_translation: Vec3::new(3.0, 0.0, 0.0),
        cuboid_half_extents: Vec3::new(1.0, 1.0, 1.0),
        should_collide: false,
    }; "sphere does not collide with cuboid")]
fn test_detect_collisions(case: DetectCollisionCase) {
    let mut app = App::new();
    app.add_message::<physics::messages::CollisionMessage>();

    let sphere_entity = app
        .world_mut()
        .spawn((
            Transform::from_translation(case.sphere_translation),
            physics::components::BoundingSphere {
                radius: case.sphere_radius,
            },
        ))
        .id();

    let cuboid_entity = app
        .world_mut()
        .spawn((
            Transform::from_translation(case.cuboid_translation),
            physics::components::BoundingCuboid {
                half_extents: case.cuboid_half_extents,
            },
        ))
        .id();

    app.add_systems(Update, physics::systems::detect_collisions);
    app.update();

    let collision_messages = app
        .world()
        .resource::<Messages<physics::messages::CollisionMessage>>();
    let mut collision_cursor = collision_messages.get_cursor();
    let collided = collision_cursor.read(collision_messages).any(|message| {
        (message.a == sphere_entity && message.b == cuboid_entity)
            || (message.a == cuboid_entity && message.b == sphere_entity)
    });

    assert_eq!(collided, case.should_collide);
}

#[derive(Default)]
struct ResolveSphereAabbCollisionCase {
    initial_velocity: Vec3,
    initial_position: Vec3,
    normal: Vec3,
    penetration: f32,
    expected_velocity: Vec3,
    expected_position: Vec3,
}

#[test_case(
    ResolveSphereAabbCollisionCase {
        initial_velocity: Vec3::new(0.0, 0.0, -1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        expected_velocity: Vec3::new(0.0, 0.0, 1.0),
        ..default()
    }; "reflects negative z velocity")]
#[test_case(
    ResolveSphereAabbCollisionCase {
        initial_velocity: Vec3::new(0.0, 0.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        expected_velocity: Vec3::new(0.0, 0.0, -1.0),
        ..default()
    }; "reflects positive z velocity")]
#[test_case(
    ResolveSphereAabbCollisionCase {
        initial_position: Vec3::new(1.0, 2.0, 3.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        penetration: 1.0,
        expected_position: Vec3::new(1.0, 2.0, 2.0),
        ..default()
    }; "Moves transform out of collision manually")]
fn test_resolve_sphere_aabb_collision(case: ResolveSphereAabbCollisionCase) {
    let mut app = App::new();
    app.add_message::<physics::messages::CollisionMessage>();

    let sphere_entity = app
        .world_mut()
        .spawn((
            physics::components::Velocity(case.initial_velocity),
            physics::components::BoundingSphere { radius: 1.0 },
            Transform::from_translation(case.initial_position),
        ))
        .id();

    let cuboid_entity = app
        .world_mut()
        .spawn((physics::components::BoundingCuboid {
            half_extents: Vec3::new(1.0, 1.0, 1.0),
        },))
        .id();
    let collision_message = physics::messages::CollisionMessage {
        a: sphere_entity,
        b: cuboid_entity,
        normal: case.normal,
        contact_point: Vec3::default(),
        penetration: case.penetration,
    };
    let mut messages = app
        .world_mut()
        .resource_mut::<Messages<physics::messages::CollisionMessage>>();
    messages.write(collision_message);

    app.add_systems(Update, physics::systems::resolve_sphere_aabb_collision);
    app.update();

    let velocity = app
        .world()
        .get::<physics::components::Velocity>(sphere_entity)
        .unwrap();
    assert_eq!(velocity.0, case.expected_velocity);
    let transform = app.world().get::<Transform>(sphere_entity).unwrap();
    assert_eq!(transform.translation, case.expected_position)
}
