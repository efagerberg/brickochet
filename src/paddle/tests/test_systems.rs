use bevy::prelude::*;
use test_case::test_case;

use crate::{ball, paddle, physics, playfield};

const SENSITIVITY: f32 = 0.025;
const PLAYFIELD_HALF: f32 = 5.0;
const PADDLE_HALF: f32 = 1.0;

fn base_app() -> App {
    let mut app = App::new();
    app.add_message::<bevy::input::mouse::MouseMotion>();

    app.world_mut().spawn((
        playfield::components::Goal::Enemy,
        physics::components::BoundingCuboid {
            half_extents: Vec3::new(PLAYFIELD_HALF, PLAYFIELD_HALF, 0.5),
        },
    ));
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
            physics::components::BoundingCuboid {
                half_extents: Vec3::new(PADDLE_HALF, PADDLE_HALF, 0.5),
            },
            Transform {
                translation: case.starting_position,
                ..default()
            },
        ))
        .id();

    let mut messages = app
        .world_mut()
        .resource_mut::<Messages<bevy::input::mouse::MouseMotion>>();
    for delta in case.motion_deltas {
        messages.write(bevy::input::mouse::MouseMotion { delta });
    }

    app.update();

    let transform = app.world().get::<Transform>(paddle_entity).unwrap();
    assert_eq!(transform.translation, case.expected_position);
}

enum PaddleImpactModifierSetupScenario {
    Collision,
    NoCollision,
    CollisionForMissingEntities,
}

struct ApplyPaddleImpactModifierCase {
    scenario: PaddleImpactModifierSetupScenario,
    z_speed_delta: f32,
    initial_velocity: Vec3,
    expected_z_velocity: f32,
}

#[test_case(
    ApplyPaddleImpactModifierCase {
        scenario: PaddleImpactModifierSetupScenario::Collision,
        z_speed_delta: 0.5,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 5.5,
    }
    ; "positive velocity increases"
)]
#[test_case(
    ApplyPaddleImpactModifierCase {
        scenario: PaddleImpactModifierSetupScenario::Collision,
        z_speed_delta: 0.5,
        initial_velocity: Vec3::new(0.0, 0.0, -5.0),
        expected_z_velocity: -5.5,
    }
    ; "negative velocity increases"
)]
#[test_case(
    ApplyPaddleImpactModifierCase {
        scenario: PaddleImpactModifierSetupScenario::Collision,
        z_speed_delta: -0.5,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 4.5,
    }
    ; "positive velocity decreases"
)]
#[test_case(
    ApplyPaddleImpactModifierCase {
        scenario: PaddleImpactModifierSetupScenario::Collision,
        z_speed_delta: -0.5,
        initial_velocity: Vec3::new(0.0, 0.0, -5.0),
        expected_z_velocity: -4.5,
    }
    ; "negative velocity decreases"
)]
#[test_case(
    ApplyPaddleImpactModifierCase {
        scenario: PaddleImpactModifierSetupScenario::NoCollision,
        z_speed_delta: 1.0,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 5.0,
    }
    ; "no change"
)]
#[test_case(
    ApplyPaddleImpactModifierCase {
        scenario: PaddleImpactModifierSetupScenario::CollisionForMissingEntities,
        z_speed_delta: 1.0,
        initial_velocity: Vec3::new(0.0, 0.0, 5.0),
        expected_z_velocity: 5.0,
    }
    ; "message does not refer to paddle or sphere entities"
)]
fn test_apply_paddle_impact_modifiers(case: ApplyPaddleImpactModifierCase) {
    let mut app = App::new();
    app.add_message::<physics::messages::CollisionMessage>();
    app.add_systems(Update, paddle::systems::apply_paddle_impact_modifiers);
    let time: Time = Time::default();
    app.insert_resource(time);

    let sphere_entity = app
        .world_mut()
        .spawn((
            physics::components::Velocity(case.initial_velocity),
            physics::components::BoundingSphere::default(),
        ))
        .id();

    let paddle_entity = app
        .world_mut()
        .spawn((
            paddle::components::Paddle,
            paddle::components::PaddleImpactModifiers {
                z_speed_delta: case.z_speed_delta,
                ..default()
            },
        ))
        .id();

    match case.scenario {
        PaddleImpactModifierSetupScenario::Collision => {
            app.world_mut()
                .resource_mut::<Messages<physics::messages::CollisionMessage>>()
                .write(physics::messages::CollisionMessage {
                    a: sphere_entity,
                    b: paddle_entity,
                    normal: Vec3::default(),
                    contact_point: Vec3::default(),
                    penetration: 0.0,
                });
        }
        PaddleImpactModifierSetupScenario::NoCollision => (),
        PaddleImpactModifierSetupScenario::CollisionForMissingEntities => {
            let non_sphere = app.world_mut().spawn_empty().id();
            let non_paddle = app.world_mut().spawn_empty().id();
            app.world_mut()
                .resource_mut::<Messages<physics::messages::CollisionMessage>>()
                .write(physics::messages::CollisionMessage {
                    a: non_sphere,
                    b: non_paddle,
                    normal: Vec3::default(),
                    contact_point: Vec3::default(),
                    penetration: 0.0,
                });
        }
    }

    app.update();

    let velocity = app
        .world()
        .get::<physics::components::Velocity>(sphere_entity)
        .unwrap();

    assert_eq!(velocity.0.z, case.expected_z_velocity);
}

struct InitializePaddleMotionCase {
    start_pos: Vec2,
    pending: bool,
    expected_start_pos: Vec2,
    expected_start_time: f32,
    expected_pending: bool,
}

#[test_case(
    InitializePaddleMotionCase {
        start_pos: Vec2::ZERO,
        pending: false,
        expected_start_pos: Vec2::ZERO,
        expected_start_time: 0.0,
        expected_pending: true,
    }
    ; "initializes motion record"
)]
#[test_case(
    InitializePaddleMotionCase {
        start_pos: Vec2::new(1.0, 1.0),
        pending: false,
        expected_start_pos: Vec2::new(1.0, 1.0),
        expected_start_time: 0.0,
        expected_pending: true,
    }
    ; "initializes motion record with non-zero position"
)]
fn test_initialize_paddle_motion(case: InitializePaddleMotionCase) {
    let mut app = App::new();
    app.add_message::<physics::messages::CollisionMessage>();
    app.add_systems(Update, paddle::systems::initialize_paddle_motion);
    let time: Time = Time::default();
    app.insert_resource(time.clone());

    let paddle_entity = app
        .world_mut()
        .spawn((
            paddle::components::Paddle,
            Transform {
                translation: Vec3::new(case.start_pos.x, case.start_pos.y, 0.0),
                ..default()
            },
            paddle::components::PaddleMotionRecord {
                start_pos: Vec2::ZERO,
                start_time: 0.0,
                pending: case.pending,
                ..default()
            },
        ))
        .id();

    let sphere_entity = app.world_mut().spawn_empty().id();

    app.world_mut()
        .resource_mut::<Messages<physics::messages::CollisionMessage>>()
        .write(physics::messages::CollisionMessage {
            a: sphere_entity,
            b: paddle_entity,
            normal: Vec3::default(),
            contact_point: Vec3::default(),
            penetration: 0.0,
        });

    app.update();

    let record = app
        .world()
        .get::<paddle::components::PaddleMotionRecord>(paddle_entity)
        .unwrap();

    assert_eq!(record.start_pos, case.expected_start_pos);
    assert_eq!(record.start_time, case.expected_start_time);
    assert_eq!(record.pending, case.expected_pending);
}

struct FinalizePaddleMotionCase {
    start_pos: Vec2,
    start_time: f32,
    pending: bool,
    current_pos: Vec3,
    advance_time: f32,
    expected_delta: Vec2,
    expected_pending: bool,
}

#[test_case(
    FinalizePaddleMotionCase {
        start_pos: Vec2::ZERO,
        start_time: 0.0,
        pending: true,
        current_pos: Vec3::new(2.0, 1.0, 0.0),
        advance_time: 0.31,
        expected_delta: Vec2::new(0.02, 0.02),
        expected_pending: false,
    }
    ; "computes normalized delta after threshold"
)]
#[test_case(
    FinalizePaddleMotionCase {
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
    FinalizePaddleMotionCase {
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
fn test_finalize_paddle_motion(case: FinalizePaddleMotionCase) {
    let mut app = App::new();

    app.world_mut().spawn(Window {
        resolution: (100, 50).into(),
        ..default()
    });

    app.add_systems(Update, paddle::systems::finalize_paddle_motion);

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
        motion_delta: Vec2::ZERO,
        pending: false,
        expected_curve: Vec2::ZERO,
    }
    ; "zero delta record does nothing"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(-5.0, -5.0),
        pending: false,
        expected_curve: Vec2::new(3.0, 3.0),
    }
    ; "super curve negative"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(-2.0, -2.0),
        pending: false,
        expected_curve: Vec2::new(1.0, 1.0),
    }
    ; "normal curve negative"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(2.0, 2.0),
        pending: false,
        expected_curve: Vec2::new(-1.0, -1.0),
    }
    ; "normal curve positive"
)]
#[test_case(
    ApplyCurveCase {
        motion_delta: Vec2::new(5.0, 5.0),
        pending: false,
        expected_curve: Vec2::new(-3.0, -3.0),
    }
    ; "super curve positive"
)]
fn test_apply_curve_from_motion_record(case: ApplyCurveCase) {
    let mut app = App::new();
    app.add_systems(Update, paddle::systems::apply_curve_from_motion_record);

    let sphere_entity = app
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
            z_speed_delta: 0.0,
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
        .get::<physics::components::Curve>(sphere_entity)
        .unwrap();

    assert_eq!(curve.0, case.expected_curve);
}
