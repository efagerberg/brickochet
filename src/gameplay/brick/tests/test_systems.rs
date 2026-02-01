use bevy::prelude::*;

use crate::gameplay::{brick, playfield};
use crate::physics;

use test_case::test_case;

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
