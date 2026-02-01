use bevy::prelude::*;

use crate::gameplay::{brick, playfield};
use crate::health;
use crate::physics;
use crate::rendering;
use crate::test_utils;

use test_case::test_case;

struct UpdateHealthColorCase {
    health: health::components::Health,
    expected_color: Option<Color>,
}

#[test_case(
    UpdateHealthColorCase {
        health: health::components::Health {
            current: 10,
            max: 10,
        },
        expected_color: Some(Color::linear_rgb(0.0, 1.0, 0.0))
    }; "green when health is still full"
)]
#[test_case(
    UpdateHealthColorCase {
        health: health::components::Health {
            current: 1,
            max: 3,
        },
        expected_color: Some(Color::linear_rgb(1.0, 0.0, 0.0))
    }; "red when 1 hp left"
)]
#[test_case(
    UpdateHealthColorCase {
        health: health::components::Health {
            current: 0,
            max: 3,
        },
        expected_color: None
    }; "no message when 0 hp"
)]
fn test_update_health_color(case: UpdateHealthColorCase) {
    let mut app = App::new();
    app.add_message::<health::messages::HealChangedMessage>();
    app.add_message::<rendering::messages::MaterialColorsChangedMessage>();
    app.add_systems(Update, brick::systems::update_health_color);
    let entity = app
        .world_mut()
        .spawn((case.health, brick::components::Brick))
        .id();
    let mut writer = app
        .world_mut()
        .resource_mut::<Messages<health::messages::HealChangedMessage>>();
    writer.write(health::messages::HealChangedMessage { entity, delta: 0 });
    app.update();

    let mut expected: Vec<rendering::messages::MaterialColorsChangedMessage> = vec![];
    if case.expected_color.is_some() {
        expected.push(rendering::messages::MaterialColorsChangedMessage {
            entity,
            base_color: case.expected_color,
            emissive: None,
        })
    };
    test_utils::assertions::assert_messages(&app, &expected);
}

#[test]
fn test_handle_collision_decreases_health() {
    let mut app = App::new();
    app.add_message::<health::messages::HealChangedMessage>();
    app.add_message::<physics::messages::CollisionMessage>();
    app.add_systems(Update, brick::systems::handle_collision);
    let entity = app
        .world_mut()
        .spawn((
            health::components::Health { current: 1, max: 1 },
            brick::components::Brick,
        ))
        .id();
    let collided_with = app.world_mut().spawn_empty().id();

    let mut writer = app
        .world_mut()
        .resource_mut::<Messages<physics::messages::CollisionMessage>>();
    writer.write(physics::messages::CollisionMessage {
        a: collided_with,
        b: entity,
        normal: Vec3::ZERO,
        contact_point: Vec3::ZERO,
        penetration: 0.0,
    });
    app.update();

    let expected = health::messages::HealChangedMessage { entity, delta: -1 };
    test_utils::assertions::assert_messages(&app, &vec![expected]);
}

struct SpawnBrickWallCase {
    goal_size: Vec3,
    brick_size: Vec3,
    expected_brick_positions: Vec<Vec3>,
}

#[test_case(
    SpawnBrickWallCase {
        goal_size: Vec3::new(1.0, 1.0, 0.1),
        brick_size: Vec3::new(1.0, 1.0, 0.1),
        expected_brick_positions: vec![Vec3::new(0.0, 0.0, 0.2)]
    }
    ; "spawns a single brick when wall is the brick size"
)]
#[test_case(
    SpawnBrickWallCase {
        goal_size: Vec3::new(2.0, 2.0, 0.1),
        brick_size: Vec3::new(1.0, 1.0, 0.1),
        expected_brick_positions: vec![Vec3::new(-0.5, -0.5, 0.2), Vec3::new(0.5, -0.5, 0.2), Vec3::new(-0.5, 0.5, 0.2), Vec3::new(0.5, 0.5, 0.2)]
    }
    ; "spawns 4 brick when wall is 2x the brick size"
)]
#[test_case(
    SpawnBrickWallCase {
        goal_size: Vec3::new(1.5, 1.5, 0.1),
        brick_size: Vec3::new(1.0, 1.0, 0.1),
        expected_brick_positions: vec![Vec3::new(0.0, 0.0, 0.2)]
    }
    ; "spawns 1 brick when wall is 1.5x the brick size"
)]
#[test_case(
    SpawnBrickWallCase {
        goal_size: Vec3::new(2.0, 1.5, 0.1),
        brick_size: Vec3::new(1.0, 1.0, 0.1),
        expected_brick_positions: vec![Vec3::new(-0.5, 0.0, 0.2), Vec3::new(0.5, 0.0, 0.2)]
    }
    ; "spawns 2 brick row when wall width is 2x brick size, but height is 1x"
)]
#[test_case(
    SpawnBrickWallCase {
        goal_size: Vec3::new(1.5, 2.0, 0.1),
        brick_size: Vec3::new(1.0, 1.0, 0.1),
        expected_brick_positions: vec![Vec3::new(0.0, -0.5, 0.2), Vec3::new(0.0, 0.5, 0.2)]
    }
    ; "spawns 2 brick column when wall width is 1x brick size, but height is 2x"
)]
fn test_spawn_brick_wall(case: SpawnBrickWallCase) {
    let mut app = App::new();
    app.insert_resource(playfield::resources::Playfield {
        brick_size: case.brick_size,
        ..default()
    });
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());

    app.world_mut().spawn((
        playfield::components::Goal::Enemy,
        Transform::default(),
        physics::components::BoundingCuboid {
            half_extents: case.goal_size * 0.5,
        },
    ));
    app.add_systems(Update, brick::systems::spawn_brick_wall);
    app.update();

    let mut brick_query = app.world_mut().query_filtered::<(
        &Transform,
        &physics::components::BoundingCuboid,
    ), With<brick::components::Brick>>();

    let bricks: Vec<(&Transform, &physics::components::BoundingCuboid)> =
        brick_query.iter(&app.world()).collect();

    assert_eq!(bricks.len(), case.expected_brick_positions.len());
    for ((transform, bounding_cuboid), expected_brick_position) in
        bricks.into_iter().zip(case.expected_brick_positions.iter())
    {
        assert_eq!(bounding_cuboid.half_extents, case.brick_size * 0.5);
        assert_eq!(transform.translation, *expected_brick_position);
    }
}
