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

    // app.update() for some reason was not triggering the system
    app.update();

    let transform = app.world().get::<Transform>(entity).unwrap();
    assert_eq!(transform.translation, case.expected_translation);
}

#[derive(Default)]
struct ApplyCurveCase {
    initial_velocity: Vec3,
    curve: Vec2,
    delta_secs: f32,
    expected_velocity: Vec3,
}

#[test_case(
    ApplyCurveCase {
        initial_velocity: Vec3::new(1.0, 1.0, 0.0),
        curve: Vec2::new(0.5, -0.25),
        delta_secs: 1.0,
        expected_velocity: Vec3::new(1.5, 0.75, 0.0),
    }
; "curve modifies x and y velocity")]
#[test_case(
    ApplyCurveCase {
        initial_velocity: Vec3::ZERO,
        curve: Vec2::new(1.0, 1.0),
        delta_secs: 1.0,
        expected_velocity: Vec3::new(1.0, 1.0, 0.0),
    }
; "curve applied to zero velocity")]
fn test_apply_curve_modifies_velocity(case: ApplyCurveCase) {
    let mut app = App::new();

    let entity = app
        .world_mut()
        .spawn((
            physics::components::Velocity(case.initial_velocity),
            physics::components::Curve(case.curve),
        ))
        .id();

    app.add_systems(Update, physics::systems::apply_curve);

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

struct ReflectSphereCase {
    initial_velocity: Vec3,
    expected_velocity: Vec3,
}

#[test_case(
    ReflectSphereCase {
        initial_velocity: Vec3::new(0.0, 0.0, -1.0),
        expected_velocity: Vec3::new(0.0, 0.0, 1.0),
    }; "reflects negative z velocity")]
#[test_case(
    ReflectSphereCase {
        initial_velocity: Vec3::new(0.0, 0.0, 1.0),
        expected_velocity: Vec3::new(0.0, 0.0, -1.0),
    }; "reflects positive z velocity")]
fn test_reflect_sphere(case: ReflectSphereCase) {
    let mut app = App::new();
    app.add_message::<physics::messages::CollisionMessage>();

    let sphere_entity = app
        .world_mut()
        .spawn((
            physics::components::Velocity(case.initial_velocity),
            physics::components::BoundingSphere { radius: 1.0 },
        ))
        .id();

    let cuboid_entity = app.world_mut().spawn_empty().id();
    let collision_message = physics::messages::CollisionMessage {
        a: sphere_entity,
        b: cuboid_entity,
    };
    let mut messages = app
        .world_mut()
        .resource_mut::<Messages<physics::messages::CollisionMessage>>();
    messages.write(collision_message);

    app.add_systems(Update, physics::systems::reflect_sphere);
    app.update();

    let velocity = app
        .world()
        .get::<physics::components::Velocity>(sphere_entity)
        .unwrap();
    assert_eq!(velocity.0, case.expected_velocity);
}
