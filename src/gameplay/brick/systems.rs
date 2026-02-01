use bevy::prelude::*;

use crate::gameplay::{brick, playfield};
use crate::{health, physics};

pub fn spawn_brick_wall(
    mut commands: Commands,
    goal_query: Query<(
        &playfield::components::Goal,
        &Transform,
        &physics::components::BoundingCuboid,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    playfield: Res<playfield::resources::Playfield>,
) {
    let (_, enemy_goal_transform, enemy_goal_bounds) = goal_query
        .iter()
        .find(|(goal, _, __)| **goal == playfield::components::Goal::Enemy)
        .expect("Missing enemy goal, cannot spawn brick wall");

    // Dimensions of the wall
    let wall_width = enemy_goal_bounds.half_extents.x * 2.0;
    let wall_height = enemy_goal_bounds.half_extents.y * 2.0;
    let wall_depth = enemy_goal_bounds.half_extents.z * 2.0;

    // Brick size (uniform)
    let brick_size = playfield.brick_size;

    // How many bricks fit
    let bricks_x = (wall_width / brick_size.x).floor() as i32;
    let bricks_y = (wall_height / brick_size.y).floor() as i32;

    // Total grid size
    let total_width = bricks_x as f32 * brick_size.x;
    let total_height = bricks_y as f32 * brick_size.y;

    let total_bricks = bricks_x * bricks_y;
    for index in 0..total_bricks {
        let x = index % bricks_x;
        let y = index / bricks_x;

        let pos = Vec3::new(
            enemy_goal_transform.translation.x - total_width * 0.5 + x as f32 * brick_size.x,
            enemy_goal_transform.translation.y - total_height * 0.5 + y as f32 * brick_size.y,
            enemy_goal_transform.translation.z + wall_depth + brick_size.z,
        );

        let color = Color::linear_rgb(0.0, 1.0, 0.0);
        spawn_brick(
            &mut commands,
            &mut meshes,
            &mut materials,
            pos,
            brick_size,
            color,
        );
    }
}

fn spawn_brick(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    size: Vec3,
    color: Color,
) {
    // Outer black border (slightly larger)
    let border_padding = 0.25;

    let border = commands
        .spawn((
            Name::new("Brick Border"),
            Mesh3d(meshes.add(Cuboid::new(
                size.x + border_padding,
                size.y + border_padding,
                size.z * 0.05, // thin
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::BLACK,
                unlit: true,
                ..default()
            })),
        ))
        .id();

    // Main colored brick
    let main = commands
        .spawn((
            Name::new("Brick"),
            brick::components::Brick,
            physics::components::BoundingCuboid {
                half_extents: size * 0.5,
            },
            Transform::from_translation(Vec3::new(
                position.x + size.x * 0.5,
                position.y + size.y * 0.5,
                position.z,
            )),
            GlobalTransform::default(),
            health::components::Health { max: 3, current: 3 },
            Mesh3d(meshes.add(Cuboid::new(
                size.x - border_padding,
                size.y - border_padding,
                size.z,
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            health::components::HealthColors {
                max: LinearRgba::rgb(0.0, 1.0, 0.0),
                min: LinearRgba::rgb(1.0, 0.0, 0.0),
            },
        ))
        .id();
    commands.entity(main).add_child(border);
}

pub fn handle_collision(
    brick_query: Query<
        Entity,
        (
            With<brick::components::Brick>,
            With<health::components::Health>,
        ),
    >,
    mut collision_messages: MessageReader<physics::messages::CollisionMessage>,
    mut health_changed_messages: MessageWriter<health::messages::HealChangedMessage>,
) {
    for message in collision_messages.read() {
        if let Ok(entity) = brick_query.get(message.b) {
            health_changed_messages
                .write(health::messages::HealChangedMessage { entity, delta: -1 });
        }
    }
}
