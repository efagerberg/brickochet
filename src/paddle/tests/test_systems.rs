use bevy::prelude::*;
use test_case::test_case;

use crate::{ball, paddle, physics, playfield};

const SENSITIVITY: f32 = 0.025;
const PLAYFIELD_HALF: f32 = 5.0;
const PADDLE_HALF: f32 = 1.0;

fn base_app() -> App {
    let mut app = App::new();
    app.insert_resource(playfield::resources::Playfield {
        aabb: physics::components::Aabb3d {
            half_extents: Vec3::new(PLAYFIELD_HALF, PLAYFIELD_HALF, 0.1)
        },
        ..default()
    });
    app.add_message::<bevy::input::mouse::MouseMotion>();
    app
}

struct PaddleMouseControlCase {
    starting_position: Vec3,
    motion_deltas: Vec<Vec2>,
    expected_position: Vec3,
}

#[test_case(
    PaddleMouseControlCase {
        starting_position: Vec3::ZERO,
        motion_deltas: vec![],
        expected_position: Vec3::ZERO,
    }
    ; "no mouse movement"
)]
#[test_case(
    PaddleMouseControlCase {
        starting_position: Vec3::ZERO,
        motion_deltas: vec![Vec2::new(10.0, 0.0)],
        expected_position: Vec3::new(10.0 * SENSITIVITY, 0.0, 0.0),
    }
    ; "positive x movement"
)]
#[test_case(
    PaddleMouseControlCase {
        starting_position: Vec3::ZERO,
        motion_deltas: vec![Vec2::new(-10.0, 0.0)],
        expected_position: Vec3::new(-10.0 * SENSITIVITY, 0.0, 0.0),
    }
    ; "negative x movement"
)]
#[test_case(
    PaddleMouseControlCase {
        starting_position: Vec3::ZERO,
        motion_deltas: vec![Vec2::new(0.0, 10.0)],
        expected_position: Vec3::new(0.0, -10.0 * SENSITIVITY, 0.0),
    }
    ; "positive y inverted"
)]
#[test_case(
    PaddleMouseControlCase {
        starting_position: Vec3::ZERO,
        motion_deltas: vec![Vec2::new(10_000.0, 10_000.0)],
        expected_position: Vec3::new(
            PLAYFIELD_HALF - PADDLE_HALF,
            -(PLAYFIELD_HALF - PADDLE_HALF),
            0.0,
        ),
    }
    ; "clamps both axes"
)]
fn test_paddle_mouse_control(case: PaddleMouseControlCase) {
    let mut app = base_app();
    app.add_systems(Update, paddle::systems::paddle_mouse_control);

    let paddle_entity = app
        .world_mut()
        .spawn((
            paddle::components::Paddle,
            physics::components::Aabb3d {
                half_extents: Vec3::new(PADDLE_HALF, PADDLE_HALF, 0.5,)
            },
            Transform {
                translation: case.starting_position,
                ..default()
            },
        ))
        .id();

    let mut events = app
        .world_mut()
        .resource_mut::<Messages<bevy::input::mouse::MouseMotion>>();
    for delta in case.motion_deltas {
        events.write(bevy::input::mouse::MouseMotion { delta });
    }

    app.update();

    let transform = app.world().get::<Transform>(paddle_entity).unwrap();
    assert_eq!(transform.translation, case.expected_position);
}

struct PaddleBallCollisionCase {
    ball_position: Vec3,
    paddle_position: Vec3,
    initial_velocity: Vec3,
    expected_z_velocity: f32,
}

#[test_case(
    PaddleBallCollisionCase {
        ball_position: Vec3::new(0.0, 0.0, -0.5),
        paddle_position: Vec3::ZERO,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: -5.5,
    }
    ; "valid collision"
)]
#[test_case(
    PaddleBallCollisionCase {
        ball_position: Vec3::new(2.0, 0.0, -0.5),
        paddle_position: Vec3::ZERO,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 5.0,
    }
    ; "miss on x axis"
)]
#[test_case(
    PaddleBallCollisionCase {
        ball_position: Vec3::new(0.0, 0.0, -5.0),
        paddle_position: Vec3::ZERO,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 5.0,
    }
    ; "miss z band"
)]
#[test_case(
    PaddleBallCollisionCase {
        ball_position: Vec3::new(0.0, 0.0, -0.5),
        paddle_position: Vec3::ZERO,
        initial_velocity: Vec3::new(0.0, 0.0, -5.0),
        expected_z_velocity: -5.0,
    }
    ; "moving away"
)]
#[test_case(
    PaddleBallCollisionCase {
        ball_position: Vec3::new(0.0, 2.0, -0.5),
        paddle_position: Vec3::ZERO,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 5.0, // no collision
    }
    ; "miss on y axis"
)]
#[test_case(
    PaddleBallCollisionCase {
        ball_position: Vec3::new(0.0, 0.0, -5.0),
        paddle_position: Vec3::ZERO,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 5.0, // no collision
    }
    ; "z below min"
)]
#[test_case(
    PaddleBallCollisionCase {
        ball_position: Vec3::new(0.0, 0.0, 5.0),
        paddle_position: Vec3::ZERO,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 5.0, // no collision
    }
    ; "z above max"
)]
fn test_paddle_ball_collision(case: PaddleBallCollisionCase) {
    let mut app = App::new();
    app.add_systems(Update, paddle::systems::paddle_ball_collision);
    let time: Time = Time::default();
    app.insert_resource(time);

    let ball_entity = app
        .world_mut()
        .spawn((
            ball::components::BallModifiers::starting(),
            Transform {
                translation: case.ball_position,
                ..default()
            },
            physics::components::Velocity(case.initial_velocity),
        ))
        .id();

    app.world_mut().spawn((
        paddle::components::Paddle,
        physics::components::Aabb3d {
            half_extents: Vec3::new(PADDLE_HALF, PADDLE_HALF, 1.0),
        },
        paddle::components::PaddleImpactModifiers {
            contact_z_speed_increase: 0.5,
            ..default()
        },
        paddle::components::PaddleMotionRecord::default(),
        Transform {
            translation: case.paddle_position,
            ..default()
        },
    ));

    app.update();

    let velocity = app
        .world()
        .get::<physics::components::Velocity>(ball_entity)
        .unwrap();

    assert_eq!(velocity.0.z, case.expected_z_velocity);
}

struct RecordPaddleMotionCase {
    start_pos: Vec2,
    start_time: f32,
    pending: bool,
    current_pos: Vec3,
    advance_time: f32,
    expected_delta: Vec2,
    expected_pending: bool,
}

#[test_case(
    RecordPaddleMotionCase {
        start_pos: Vec2::ZERO,
        start_time: 0.0,
        pending: true,
        current_pos: Vec3::new(2.0, 1.0, 0.0),
        advance_time: 0.31,
        expected_delta: Vec2::new(2.0, 1.0),
        expected_pending: false,
    }
    ; "computes delta after threshold"
)]
#[test_case(
    RecordPaddleMotionCase {
        start_pos: Vec2::ZERO,
        start_time: 0.0,
        pending: true,
        current_pos: Vec3::new(2.0, 1.0, 0.0),
        advance_time: 0.1,
        expected_delta: Vec2::ZERO,
        expected_pending: true,
    }
    ; "does nothing before threshold"
)]
#[test_case(
    RecordPaddleMotionCase {
        start_pos: Vec2::ZERO,
        start_time: 0.0,
        pending: false,
        current_pos: Vec3::new(2.0, 1.0, 0.0),
        advance_time: 0.31,
        expected_delta: Vec2::ZERO,
        expected_pending: false,
    }
    ; "pending false does not update delta"
)]
fn test_record_paddle_motion(case: RecordPaddleMotionCase) {
    let mut app = App::new();
    app.add_systems(Update, paddle::systems::record_paddle_motion);

    let mut time: Time = Time::default();
    app.insert_resource(time.clone());

    let entity = app
        .world_mut()
        .spawn((
            paddle::components::Paddle,
            Transform {
                translation: case.current_pos,
                ..default()
            },
            paddle::components::PaddleMotionRecord {
                start_pos: case.start_pos,
                start_time: case.start_time,
                pending: case.pending,
                delta: Vec2::ZERO,
            },
        ))
        .id();

    time.advance_by(std::time::Duration::from_secs_f32(case.advance_time));
    *app.world_mut().resource_mut::<Time>() = time;

    app.update();

    let record = app
        .world()
        .get::<paddle::components::PaddleMotionRecord>(entity)
        .unwrap();

    assert_eq!(record.delta, case.expected_delta);
    assert_eq!(record.pending, case.expected_pending);
}

struct ApplyCurveCase {
    motion_delta: Vec2,
    pending: bool,
    expected_curve: Vec2,
}

#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(0.5, 0.5),
        pending: false,
        expected_curve: Vec2::ZERO,
    }
    ; "below threshold"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(2.0, 0.0),
        pending: false,
        expected_curve: Vec2::new(-1.0, 0.0),
    }
    ; "normal x curve"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(5.0, 0.0),
        pending: false,
        expected_curve: Vec2::new(-3.0, 0.0),
    }
    ; "super x curve"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(2.0, -2.0),
        pending: false,
        expected_curve: Vec2::new(-1.0, 1.0),
    }
    ; "both axes normal"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(5.0, 0.0),
        pending: true,
        expected_curve: Vec2::ZERO,
    }
    ; "pending record does nothing"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(-5.0, -5.0),
        pending: false,
        expected_curve: Vec2::new(3.0, 3.0), // super curve negative
    }
    ; "super curve negative"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(-2.0, -2.0),
        pending: false,
        expected_curve: Vec2::new(1.0, 1.0), // normal curve negative
    }
    ; "normal curve negative"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(2.0, 2.0),
        pending: false,
        expected_curve: Vec2::new(-1.0, -1.0), // normal curve positive
    }
    ; "normal curve positive"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(5.0, 5.0),
        pending: false,
        expected_curve: Vec2::new(-3.0, -3.0), // super curve positive
    }
    ; "super curve positive"
)]
fn test_apply_curve_from_motion_record(case: ApplyCurveCase) {
    let mut app = App::new();
    app.add_systems(Update, paddle::systems::apply_curve_from_motion_record);

    let ball_entity = app
        .world_mut()
        .spawn((
            ball::components::BallModifiers::starting(),
            physics::components::Curve(Vec2::ZERO),
        ))
        .id();

    app.world_mut().spawn((
        paddle::components::Paddle,
        paddle::components::PaddleImpactModifiers {
            normal_curve_scale: 1.0,
            super_curve_scale: 3.0,
            normal_curve_position_delta_threshold: 1.0,
            super_curve_position_delta_threshold: 4.0,
            contact_z_speed_increase: 0.0,
        },
        paddle::components::PaddleMotionRecord {
            delta: case.motion_delta,
            pending: case.pending,
            ..default()
        },
    ));

    app.update();

    let curve = app
        .world()
        .get::<physics::components::Curve>(ball_entity)
        .unwrap();

    assert_eq!(curve.0, case.expected_curve);
}
