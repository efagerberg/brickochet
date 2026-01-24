use bevy::prelude::*;

use crate::{ball, physics, playfield, rendering};

const PLAYFIELD_RES: playfield::resources::Playfield = playfield::resources::Playfield {
    wall_line_default_color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
    wall_line_highlight_color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
};

#[test]
fn test_updates_emissive_based_on_distance() {
    let (ball_transform, lines_transform, material_color) = setup_depth_lines(
        2.0,
        0.0,
        &PLAYFIELD_RES,
        PLAYFIELD_RES.wall_line_default_color,
    );
    let expected_color = LinearRgba::mix(
        &PLAYFIELD_RES.wall_line_default_color,
        &PLAYFIELD_RES.wall_line_highlight_color,
        (lines_transform.translation.z - ball_transform.translation.z)
            .abs()
            .clamp(0.0, 1.0),
    );

    assert_eq!(material_color.emissive.unwrap(), expected_color);
}

#[test]
fn test_emissive_does_not_change_if_already_highlighted_and_z_position_equal_ball_z() {
    let (_, __, material_color) = setup_depth_lines(
        0.0,
        0.0,
        &PLAYFIELD_RES,
        PLAYFIELD_RES.wall_line_highlight_color,
    );

    assert_eq!(
        material_color.emissive.unwrap(),
        PLAYFIELD_RES.wall_line_highlight_color
    );
}

fn make_transform(z: f32) -> Transform {
    Transform::from_translation(Vec3::new(0.0, 0.0, z))
}

fn setup_depth_lines(
    ball_z: f32,
    lines_z: f32,
    playfield_res: &playfield::resources::Playfield,
    initial_emissive: LinearRgba,
) -> (
    Transform,
    Transform,
    rendering::components::MaterialColorsUpdate,
) {
    let mut app = App::new();
    app.insert_resource(playfield_res.clone());
    let ball_transform = make_transform(ball_z);
    let ball_modifiers = ball::components::BallModifiers::starting();
    app.world_mut().spawn((
        ball_modifiers.clone(),
        ball_transform,
        physics::components::BoundingSphere {
            radius: ball_modifiers.base_radius,
        },
    ));
    let lines_transform = make_transform(lines_z);
    let lines_entity = app
        .world_mut()
        .spawn((
            playfield::components::DepthLines,
            lines_transform,
            rendering::components::MaterialColorsUpdate {
                base_color: None,
                emissive: Some(initial_emissive),
            },
        ))
        .id();
    app.add_systems(Update, playfield::systems::highlight_depth_lines);
    app.update();
    let lines_material_color = app
        .world()
        .get::<rendering::components::MaterialColorsUpdate>(lines_entity)
        .unwrap()
        .clone();
    (ball_transform, lines_transform, lines_material_color)
}

use std::f32::EPSILON;
use test_case::test_case;

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
    "when ball hits enemy goal, curve is cleared but velocity is unchanged"
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
    "when ball hits player goal, position is reset, velocity is set to starting velocity, and curve is cleared"
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
    "when ball is not near any wall, position, velocity, and curve are unchanged"
)]
fn test_wall_collision_handler(case: WallCollisionHandlerCase) {
    let mut app = App::new();
    let (ball_entity, _) = setup_potential_wall_collision(
        &mut app,
        case.position,
        case.velocity,
        case.curve,
        case.colliding_goal,
    );

    app.add_systems(Update, playfield::systems::handle_wall_collision);
    app.update();

    let transform = app.world().get::<Transform>(ball_entity).unwrap();
    let velocity = app
        .world()
        .get::<physics::components::Velocity>(ball_entity)
        .unwrap();
    let curve = app
        .world()
        .get::<physics::components::Curve>(ball_entity)
        .unwrap();

    assert!(
        (transform.translation - case.expected_position).length() < EPSILON,
        "expected pos {:?}, got {:?}",
        case.expected_position,
        transform.translation
    );

    assert!(
        (velocity.0 - case.expected_velocity).length() < EPSILON,
        "expected vel {:?}, got {:?}",
        case.expected_velocity,
        velocity.0
    );
    assert!(
        (curve.0 - case.expected_curve).length() < EPSILON,
        "expected curve {:?}, got {:?}",
        case.expected_curve,
        curve.0
    );
}

fn setup_potential_wall_collision(
    app: &mut App,
    pos: Vec3,
    vel: Vec3,
    curve: Vec2,
    colliding_goal: Option<playfield::components::Goal>,
) -> (Entity, Entity) {
    let mut ball_modifiers = ball::components::BallModifiers::starting();
    ball_modifiers.base_velocity = vel;

    let ball_entity = app
        .world_mut()
        .spawn((
            ball_modifiers.clone(),
            Transform::from_translation(pos),
            physics::components::BoundingSphere {
                radius: ball_modifiers.base_radius,
            },
            physics::components::Velocity(ball_modifiers.base_velocity),
            physics::components::Curve(curve),
        ))
        .id();
    app.add_message::<physics::messages::CollisionMessage>();

    let collided_with_entity = app
        .world_mut()
        .spawn((physics::components::BoundingCuboid {
            half_extents: Vec3::new(1.0, 1.0, 1.0),
        },))
        .id();
    if let Some(goal) = colliding_goal {
        app.world_mut()
            .commands()
            .entity(collided_with_entity)
            .insert(goal);
    }
    let mut messages = app
        .world_mut()
        .resource_mut::<Messages<physics::messages::CollisionMessage>>();
    messages.write(physics::messages::CollisionMessage {
        a: ball_entity,
        b: collided_with_entity,
        contact_point: pos,
        normal: Vec3::Z, // Assume a simple normal for testing purposes,
        penetration: 0.1,
    });

    (ball_entity, collided_with_entity)
}
