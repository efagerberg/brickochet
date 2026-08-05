use bevy::{ecs::entity, prelude::*};
use test_case::test_case;

use crate::{physics, test_utils};

#[derive(Default)]
struct ApplyVelocityCase {
    velocity: Vec3,
    delta_secs: f32,
    collision_hit: Option<physics::math::SweepHit>,
    expected_translation: Vec3,
}

#[test_case(
    ApplyVelocityCase {
        velocity: Vec3::new(1.0, 0.0, 0.0),
        delta_secs: 1.0,
        expected_translation: Vec3::new(1.0, 0.0, 0.0),
        ..default()
    }
; "moves 1 unit in x over 1 second")]
#[test_case(
    ApplyVelocityCase {
        velocity: Vec3::new(0.0, 2.0, 0.0),
        delta_secs: 0.5,
        expected_translation: Vec3::new(0.0, 1.0, 0.0),
        ..default()
    }
; "moves half velocity over half second")]
#[test_case(
    ApplyVelocityCase {
        velocity: Vec3::new(0.0, -2.0, 0.0),
        delta_secs: 1.0,
        collision_hit: Some(physics::math::SweepHit {
            t: 0.5,
            contact_point: Vec3::new(0.0, 1.0, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
        }),
        expected_translation: Vec3::new(0.0, -physics::systems::COLLISION_SEPARATION_EPSILON, 0.0),
    }
; "Continues path after a collision")]
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
    app.add_message::<physics::messages::CollisionMessage>();
    app.add_systems(Update, physics::systems::apply_velocity);

    if let Some(hit) = case.collision_hit {
        let b_entity = app.world_mut().spawn_empty().id();
        app.world_mut()
            .resource_mut::<Messages<physics::messages::CollisionMessage>>()
            .write(physics::messages::CollisionMessage {
                a: entity,
                b: b_entity,
                hit: hit,
            });
    }

    app.update();

    let transform = app.world().get::<Transform>(entity).unwrap();
    test_utils::assertions::assert_vec3_approx_eq(transform.translation, case.expected_translation);
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
        .spawn((physics::components::Curve(case.curve), Transform::default()))
        .id();

    app.add_systems(Update, physics::systems::apply_curve_spin);

    let mut time: Time = Time::default();
    time.advance_by(std::time::Duration::from_secs_f32(case.delta_secs));
    app.insert_resource(time);

    app.update();

    let transform = app.world().get::<Transform>(entity).unwrap();
    let expected = Quat::from_axis_angle(case.expected_axis, case.expected_angle_rads);
    let actual = transform.rotation;

    assert_eq!(expected, actual);
}

struct CuboidProperties {
    translation: Vec3,
    half_extents: Vec3,
    should_collide: bool,
}

struct DetectCollisionCase {
    sphere_translation: Vec3,
    sphere_radius: f32,
    sphere_velocity: Vec3,
    cuboids: Vec<CuboidProperties>,
    delta_secs: f32,
}

#[test_case(
    DetectCollisionCase {
        sphere_translation: Vec3::new(0.0, 0.0, 0.0),
        sphere_radius: 1.0,
        sphere_velocity: Vec3::new(1.0, 1.0, 1.0),
        cuboids: vec![
            CuboidProperties {
                translation: Vec3::new(1.5, 0.0, 0.0),
                half_extents: Vec3::new(1.0, 1.0, 1.0),
                should_collide: true,
            }
        ],
        delta_secs: 1.0,
    }; "sphere collides with cuboid")]
#[test_case(
    DetectCollisionCase {
        sphere_translation: Vec3::new(0.0, 0.0, 0.0),
        sphere_radius: 0.1,
        sphere_velocity: Vec3::new(1.0, 0.0, 0.0),
        cuboids: vec![
            CuboidProperties {
                // Farther along the path: sphere reaches this second.
                translation: Vec3::new(0.8, 0.0, 0.0),
                half_extents: Vec3::new(0.1, 0.1, 0.1),
                should_collide: false,
            },
            CuboidProperties {
                // Closer: sphere reaches this first
                translation: Vec3::new(0.5, 0.0, 0.0),
                half_extents: Vec3::new(0.1, 0.1, 0.1),
                should_collide: true,
            }
        ],
        delta_secs: 1.0,
    }; "only detects earliest collision (earliest last)")]
#[test_case(
    DetectCollisionCase {
        sphere_translation: Vec3::new(0.0, 0.0, 0.0),
        sphere_radius: 0.1,
        sphere_velocity: Vec3::new(1.0, 0.0, 0.0),
        cuboids: vec![
            CuboidProperties {
                // Closer: sphere reaches this first
                translation: Vec3::new(0.5, 0.0, 0.0),
                half_extents: Vec3::new(0.1, 0.1, 0.1),
                should_collide: true,
            },
            CuboidProperties {
                // Farther along the path: sphere reaches this second.
                translation: Vec3::new(0.8, 0.0, 0.0),
                half_extents: Vec3::new(0.1, 0.1, 0.1),
                should_collide: false,
            },
        ],
        delta_secs: 1.0,
    }; "only detects earliest collision (earliest first)")]
#[test_case(
    DetectCollisionCase {
        sphere_translation: Vec3::new(0.0, 0.0, 0.0),
        sphere_radius: 1.0,
        sphere_velocity: Vec3::new(1.0, 1.0, 1.0),
        cuboids: vec![
            CuboidProperties {
                translation: Vec3::new(4.0, 0.0, 0.0),
                half_extents: Vec3::new(1.0, 1.0, 1.0),
                should_collide: false,
            }
        ],
        delta_secs: 1.0,
    }; "no collision"
)]
fn test_detect_collisions(case: DetectCollisionCase) {
    let mut app = App::new();
    app.add_message::<physics::messages::CollisionMessage>();

    let sphere_entity = app
        .world_mut()
        .spawn((
            Transform::from_translation(case.sphere_translation),
            physics::components::SphereCollider {
                radius: case.sphere_radius,
            },
            physics::components::Velocity(case.sphere_velocity),
            physics::components::DynamicBody,
        ))
        .id();

    let cuboid_to_props: entity::EntityHashMap<CuboidProperties> = case
        .cuboids
        .into_iter()
        .map(|props| {
            let entity = app
                .world_mut()
                .spawn((
                    Transform::from_translation(props.translation),
                    physics::components::CuboidCollider {
                        half_extents: props.half_extents,
                    },
                ))
                .id();
            (entity, props)
        })
        .collect();

    let mut time: Time = Time::default();
    time.advance_by(std::time::Duration::from_secs_f32(case.delta_secs));
    app.insert_resource(time);
    app.add_systems(Update, physics::systems::detect_collisions);
    app.update();

    let collision_messages = app
        .world()
        .resource::<Messages<physics::messages::CollisionMessage>>();
    let mut collision_cursor = collision_messages.get_cursor();

    let collided_entities: entity::EntityHashSet = collision_cursor
        .read(collision_messages)
        .map(|message| {
            assert_eq!(
                message.a, sphere_entity,
                "collision message referenced an unexpected sphere entity"
            );
            message.b
        })
        .collect();
    for (entity, props) in &cuboid_to_props {
        assert_eq!(
            collided_entities.contains(entity),
            props.should_collide,
            "collision mismatch for cuboid {entity:?}: expected should_collide={}",
            props.should_collide
        );
    }
}

#[derive(Default)]
struct ResolveSphereAabbCollisionCase {
    initial_velocity: Option<Vec3>,
    collision_hit: Option<physics::math::SweepHit>,
    expected_velocity: Option<Vec3>,
}

#[test_case(
    ResolveSphereAabbCollisionCase {
        initial_velocity: Some(Vec3::new(1.0, 2.0, 3.0)),
        collision_hit: None,
        expected_velocity: Some(Vec3::new(1.0, 2.0, 3.0))
    };
    "Noops when no collision_data"
)]
#[test_case(
    ResolveSphereAabbCollisionCase {
        initial_velocity: Some(Vec3::new(0.0, 0.0, -1.0)),
        collision_hit: Some(physics::math::SweepHit {
            normal: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        }),
        expected_velocity: Some(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    }; "reflects negative z velocity opposite to normal")]
#[test_case(
    ResolveSphereAabbCollisionCase {
        initial_velocity: Some(Vec3::new(0.0, 0.0, 1.0)),
        collision_hit: Some(physics::math::SweepHit {
            normal: Vec3::new(0.0, 0.0, -1.0),
            ..default()
        }),
        expected_velocity: Some(Vec3::new(0.0, 0.0, -1.0)),
        ..default()
    }; "reflects positive z velocity opposite to normal")]
#[test_case(
    ResolveSphereAabbCollisionCase {
        initial_velocity: Some(Vec3::new(0.0, 0.0, 1.0)),
        collision_hit: Some(physics::math::SweepHit {
            normal: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        }),
        expected_velocity: Some(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    }; "does not reflects positive z velocity aligned to normal")]
#[test_case(
    ResolveSphereAabbCollisionCase {
        initial_velocity: None,
        collision_hit: Some(physics::math::SweepHit {
            normal: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        }),
        expected_velocity: None,
        ..default()
    }; "noops when colliding object missing velocity")]
fn test_resolve_sphere_aabb_collision_updates_velocity(case: ResolveSphereAabbCollisionCase) {
    let mut app = App::new();
    app.add_message::<physics::messages::CollisionMessage>();

    let sphere_entity = app
        .world_mut()
        .spawn((physics::components::SphereCollider { radius: 1.0 },))
        .id();
    if let Some(initial_velocity) = case.initial_velocity {
        app.world_mut()
            .entity_mut(sphere_entity)
            .insert(physics::components::Velocity(initial_velocity));
    }

    let cuboid_entity = app
        .world_mut()
        .spawn((physics::components::CuboidCollider {
            half_extents: Vec3::new(1.0, 1.0, 1.0),
        },))
        .id();
    if let Some(collision_data) = case.collision_hit {
        let collision_message = physics::messages::CollisionMessage {
            a: sphere_entity,
            b: cuboid_entity,
            hit: physics::math::SweepHit {
                normal: collision_data.normal,
                contact_point: Vec3::default(),
                t: 0.0,
            },
        };
        let mut messages = app
            .world_mut()
            .resource_mut::<Messages<physics::messages::CollisionMessage>>();
        messages.write(collision_message);
    }

    app.add_systems(Update, physics::systems::resolve_sphere_aabb_collision);
    app.update();

    if let Some(expected_velocity) = case.expected_velocity {
        let velocity = app
            .world()
            .get::<physics::components::Velocity>(sphere_entity)
            .expect("somehow missing velocity despite expecting a velocity in test case");
        assert_eq!(velocity.0, expected_velocity);
    }
}
