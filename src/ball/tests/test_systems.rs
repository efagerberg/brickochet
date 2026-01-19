use crate::{ball, physics, playfield};
use bevy::prelude::*;
use std::f32::EPSILON;
use test_case::test_case;

const PLAYFIELD_RES: playfield::resources::Playfield = playfield::resources::Playfield {
    bounds: physics::components::BoundingCuboid {
        half_extents: Vec3::new(1.0, 2.0, 3.0),
    },
    wall_line_default_color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
    wall_line_highlight_color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
};

struct WallCollisionHandlerCase {
    pos: Vec3,
    vel: Vec3,
    curve: Vec2,
    expected_pos: Vec3,
    expected_vel: Vec3,
    expected_curve: Vec2,
}

/// Helper functions for positions just beyond walls
fn just_past_far_wall() -> Vec3 {
    Vec3::new(
        0.0,
        0.0,
        -PLAYFIELD_RES.bounds.half_extents.z
            - ball::components::BallModifiers::starting().base_radius
            - 0.1,
    )
}
fn just_past_near_wall() -> Vec3 {
    Vec3::new(
        0.0,
        0.0,
        PLAYFIELD_RES.bounds.half_extents.z
            + ball::components::BallModifiers::starting().base_radius
            + 0.1,
    )
}


#[test_case(
    WallCollisionHandlerCase {
        pos: just_past_far_wall(),
        vel: -Vec3::Z,
        curve: Vec2::Y,
        expected_pos: just_past_far_wall(),
        expected_vel: -Vec3::Z,
        expected_curve: Vec2::ZERO,
    };
    "when ball hits far wall, curve is cleared but velocity is unchanged"
)]
#[test_case(
    WallCollisionHandlerCase {
        pos: just_past_near_wall(),
        vel: Vec3::Z,
        curve: -Vec2::Y,
        expected_pos: Vec3::ZERO,
        expected_vel: Vec3::Z,
        expected_curve: Vec2::ZERO,
    };
    "when ball hits near wall, position is reset, velocity is set to starting velocity, and curve is cleared"
)]
#[test_case(
    WallCollisionHandlerCase {
        pos: Vec3::ZERO,
        vel: Vec3::new(0.5, -0.5, 1.0),
        curve: Vec2::X,
        expected_pos: Vec3::ZERO,
        expected_vel: Vec3::new(0.5, -0.5, 1.0),
        expected_curve: Vec2::X,
    };
    "when ball is not near any wall, position, velocity, and curve are unchanged"
)]
fn test_wall_collision_handler(case: WallCollisionHandlerCase) {
    let mut app = App::new();
    app.insert_resource(PLAYFIELD_RES);

    let entity = setup_ball(&mut app, case.pos, case.vel, case.curve);
    app.add_systems(Update, ball::systems::wall_collision_handler);
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

fn setup_ball(app: &mut App, pos: Vec3, vel: Vec3, curve: Vec2) -> Entity {
    let mut ball_modifiers = ball::components::BallModifiers::starting();
    ball_modifiers.base_velocity = vel;

    app.world_mut()
        .spawn((
            ball_modifiers.clone(),
            Transform::from_translation(pos),
            physics::components::BoundingSphere {
                radius: ball_modifiers.base_radius,
            },
            physics::components::Velocity(ball_modifiers.base_velocity),
            physics::components::Curve(curve),
        ))
        .id()
}
