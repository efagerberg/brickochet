use bevy::prelude::*;
use test_case::test_case;

use crate::gameplay::{ball, playfield};
use crate::physics;
use crate::rendering;

#[derive(Debug)]
struct TrackBallWithDepthLineCase {
    ball_z: f32,
}

#[test_case(
    TrackBallWithDepthLineCase {
        ball_z: 0.001,
    };
    "Follows ball moving in"
)]
#[test_case(
    TrackBallWithDepthLineCase {
        ball_z: -0.001,
    };
    "Follows ball moving out"
)]
fn test_track_ball_with_depth_line_tracks_ball_as_expected(case: TrackBallWithDepthLineCase) {
    let mut app = App::new();
    let (ball_entity, line_entity) = run_tracking(&mut app, case.ball_z);

    let ball_transform = app.world().get::<Transform>(ball_entity).unwrap();
    let line_transform = app.world().get::<Transform>(line_entity).unwrap();

    assert_eq!(ball_transform.translation.z, line_transform.translation.z);
}

fn run_tracking(app: &mut App, ball_z: f32) -> (Entity, Entity) {
    let ball_modifiers = ball::components::BallModifiers::starting();
    let ball_entity = app
        .world_mut()
        .spawn((
            ball_modifiers.clone(),
            Transform::from_translation(Vec3::Z * ball_z),
            physics::components::BoundingSphere {
                radius: ball_modifiers.base_radius,
            },
        ))
        .id();

    let line_entity = app
        .world_mut()
        .spawn((
            playfield::components::DepthLine,
            Transform::from_translation(Vec3::Z),
        ))
        .id();

    app.add_message::<rendering::messages::MaterialColorsChangedMessage>();
    app.add_systems(Update, playfield::systems::track_ball_with_depth_line);

    app.update();

    (ball_entity, line_entity)
}

use std::f32::EPSILON;

struct WallCollisionHandlerCase {
    position: Vec3,
    velocity: Vec3,
    curve: Vec2,
    colliding_goal: Option<playfield::components::Goal>,
    expected_position: Vec3,
    expected_velocity: Vec3,
    expected_curve: Vec2,
}

#[test_case(
    WallCollisionHandlerCase {
        position: Vec3::new(0.0, 0.0, -1.0),
        velocity: -Vec3::Z,
        curve: Vec2::Y,
        colliding_goal: Some(playfield::components::Goal::Enemy),
        expected_position: Vec3::new(0.0, 0.0, -1.0),
        expected_velocity: -Vec3::Z,
        expected_curve: Vec2::ZERO,
    };
    "enemy goal clears curve only"
)]
#[test_case(
    WallCollisionHandlerCase {
        position: Vec3::new(0.0, 0.0, 1.0),
        velocity: Vec3::Z,
        curve: -Vec2::Y,
        colliding_goal: Some(playfield::components::Goal::Player),
        expected_position: Vec3::ZERO,
        expected_velocity: Vec3::Z,
        expected_curve: Vec2::ZERO,
    };
    "player goal resets position and clears curve"
)]
#[test_case(
    WallCollisionHandlerCase {
        position: Vec3::ZERO,
        velocity: Vec3::new(0.5, -0.5, 1.0),
        curve: Vec2::X,
        colliding_goal: None,
        expected_position: Vec3::ZERO,
        expected_velocity: Vec3::new(0.5, -0.5, 1.0),
        expected_curve: Vec2::X,
    };
    "no collision leaves ball unchanged"
)]
fn handle_wall_collision_system(case: WallCollisionHandlerCase) {
    let mut app = App::new();

    let ball_entity = setup_wall_collision_case(&mut app, &case);

    app.add_systems(Update, playfield::systems::handle_wall_collision);
    app.update();

    assert_vec3_eq(
        app.world()
            .get::<Transform>(ball_entity)
            .unwrap()
            .translation,
        case.expected_position,
        "position",
    );

    assert_vec3_eq(
        app.world()
            .get::<physics::components::Velocity>(ball_entity)
            .unwrap()
            .0,
        case.expected_velocity,
        "velocity",
    );

    assert_vec2_eq(
        app.world()
            .get::<physics::components::Curve>(ball_entity)
            .unwrap()
            .0,
        case.expected_curve,
        "curve",
    );
}

fn setup_wall_collision_case(app: &mut App, case: &WallCollisionHandlerCase) -> Entity {
    let mut modifiers = ball::components::BallModifiers::starting();
    modifiers.base_velocity = case.velocity;

    let ball_entity = app
        .world_mut()
        .spawn((
            modifiers.clone(),
            Transform::from_translation(case.position),
            physics::components::BoundingSphere {
                radius: modifiers.base_radius,
            },
            physics::components::Velocity(case.velocity),
            physics::components::Curve(case.curve),
        ))
        .id();

    app.add_message::<physics::messages::CollisionMessage>();

    let wall_entity = app
        .world_mut()
        .spawn(physics::components::BoundingCuboid {
            half_extents: Vec3::ONE,
        })
        .id();

    if let Some(goal) = case.colliding_goal {
        app.world_mut().entity_mut(wall_entity).insert(goal);
    }

    app.world_mut()
        .resource_mut::<Messages<physics::messages::CollisionMessage>>()
        .write(physics::messages::CollisionMessage {
            a: ball_entity,
            b: wall_entity,
            contact_point: case.position,
            normal: Vec3::Z,
            penetration: 0.1,
        });

    ball_entity
}

fn assert_vec3_eq(actual: Vec3, expected: Vec3, label: &str) {
    assert!(
        (actual - expected).length() < EPSILON,
        "{}: expected {:?}, got {:?}",
        label,
        expected,
        actual
    );
}

fn assert_vec2_eq(actual: Vec2, expected: Vec2, label: &str) {
    assert!(
        (actual - expected).length() < EPSILON,
        "{}: expected {:?}, got {:?}",
        label,
        expected,
        actual
    );
}
