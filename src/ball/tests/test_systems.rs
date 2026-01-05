use crate::{ball, physics, playfield};
use bevy::prelude::*;
use std::f32::EPSILON;
use test_case::test_case;

const PLAYFIELD_RES: playfield::resources::Playfield = playfield::resources::Playfield {
    half_width: 1.0,
    half_height: 2.0,
    half_depth: 3.0,
    wall_line_default_color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
    wall_line_highlight_color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
};

#[derive(Default)]
struct ReflectCase {
    pos: Vec3,
    vel: Vec3,
    curve: Vec2,
    expected_pos: Vec3,
    expected_vel: Vec3,
    expected_curve: Vec2,
}

/// Helper functions for positions just beyond walls
fn just_above_top_wall() -> Vec3 {
    Vec3::new(
        0.0,
        PLAYFIELD_RES.half_height + ball::components::RADIUS + 0.1,
        0.0,
    )
}
fn just_below_bottom_wall() -> Vec3 {
    Vec3::new(
        0.0,
        -PLAYFIELD_RES.half_height - ball::components::RADIUS - 0.1,
        0.0,
    )
}
fn just_left_of_left_wall() -> Vec3 {
    Vec3::new(
        -PLAYFIELD_RES.half_width - ball::components::RADIUS - 0.1,
        0.0,
        0.0,
    )
}
fn just_right_of_right_wall() -> Vec3 {
    Vec3::new(
        PLAYFIELD_RES.half_width + ball::components::RADIUS + 0.1,
        0.0,
        0.0,
    )
}
fn just_past_far_wall() -> Vec3 {
    Vec3::new(
        0.0,
        0.0,
        -PLAYFIELD_RES.half_depth - ball::components::RADIUS - 0.1,
    )
}
fn just_past_back_wall() -> Vec3 {
    Vec3::new(
        0.0,
        0.0,
        PLAYFIELD_RES.half_depth + ball::components::RADIUS + 0.1,
    )
}

/// Test cases using a single struct per case
#[test_case(
    ReflectCase {
        pos: just_above_top_wall(),
        vel: Vec3::Y,
        curve: Vec2::X,
        expected_pos: Vec3::new(0.0, PLAYFIELD_RES.half_height - ball::components::RADIUS, 0.0),
        expected_vel: -Vec3::Y,
        expected_curve: Vec2::X
    };
    "ball_hits_top_wall"
)]
#[test_case(
    ReflectCase {
        pos: just_below_bottom_wall(),
        vel: -Vec3::Y,
        curve: Vec2::X,
        expected_pos: Vec3::new(0.0, -PLAYFIELD_RES.half_height + ball::components::RADIUS, 0.0),
        expected_vel: Vec3::Y,
        expected_curve: Vec2::X,
    };
    "ball_hits_bottom_wall"
)]
#[test_case(
    ReflectCase {
        pos: just_left_of_left_wall(),
        vel: -Vec3::X,
        curve: Vec2::X,
        expected_pos: Vec3::new(-PLAYFIELD_RES.half_width + ball::components::RADIUS, 0.0, 0.0),
        expected_vel: Vec3::X,
        expected_curve: Vec2::X,
    };
    "ball_hits_left_wall"
)]
#[test_case(
    ReflectCase {
        pos: just_right_of_right_wall(),
        vel: Vec3::X,
        curve: Vec2::X,
        expected_pos: Vec3::new(PLAYFIELD_RES.half_width - ball::components::RADIUS, 0.0, 0.0),
        expected_vel: -Vec3::X,
        expected_curve: Vec2::X,
    };
    "ball_hits_right_wall"
)]
#[test_case(
    ReflectCase {
        pos: just_past_far_wall(),
        vel: -Vec3::Z,
        curve: Vec2::Y,
        expected_pos: Vec3::new(0.0, 0.0, -PLAYFIELD_RES.half_depth + ball::components::RADIUS),
        expected_vel: Vec3::Z,
        expected_curve: Vec2::ZERO,
    };
    "ball_hits_far_wall"
)]
#[test_case(
    ReflectCase {
        pos: just_past_back_wall(),
        vel: Vec3::Z,
        curve: -Vec2::Y,
        expected_pos: Vec3::ZERO,
        expected_vel: ball::components::DEFAULT_VELOCITY,
        expected_curve: Vec2::ZERO,
    };
    "ball_hits_back_wall"
)]
#[test_case(
    ReflectCase {
        pos: Vec3::ZERO,
        vel: Vec3::new(0.5, -0.5, 1.0),
        curve: Vec2::X,
        expected_pos: Vec3::ZERO,
        expected_vel: Vec3::new(0.5, -0.5, 1.0),
        expected_curve: Vec2::X,
    };
    "ball_inside_playfield_no_reflection"
)]
fn reflect_ball_param(case: ReflectCase) {
    let mut app = App::new();
    app.insert_resource(PLAYFIELD_RES);

    let entity = setup_ball(&mut app, case.pos, case.vel, case.curve);
    app.add_systems(Update, ball::systems::reflect_ball);
    app.update();

    let transform = app.world().get::<Transform>(entity).unwrap();
    let velocity = app
        .world()
        .get::<physics::components::Velocity>(entity)
        .unwrap();
    let curve = app
        .world()
        .get::<physics::components::Curve>(entity)
        .unwrap();

    assert!(
        (transform.translation - case.expected_pos).length() < EPSILON,
        "expected pos {:?}, got {:?}",
        case.expected_pos,
        transform.translation
    );

    assert!(
        (velocity.0 - case.expected_vel).length() < EPSILON,
        "expected vel {:?}, got {:?}",
        case.expected_vel,
        velocity.0
    );
    assert!(
        (curve.0 - case.expected_curve).length() < EPSILON,
        "expected curve {:?}, got {:?}",
        case.expected_curve,
        curve.0
    );
}

/// Helper to spawn a ball entity
fn setup_ball(app: &mut App, pos: Vec3, vel: Vec3, curve: Vec2) -> Entity {
    app.world_mut()
        .spawn((
            ball::components::Ball,
            Transform::from_translation(pos),
            physics::components::Velocity(vel),
            physics::components::Curve(curve),
        ))
        .id()
}
