use bevy::prelude::*;

use crate::components::{ball, paddle, physics};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    setup_camera(&mut commands);
    setup_lighting(&mut commands);
    spawn_paddle(&mut commands, &mut meshes, &mut materials);
    spawn_ball(&mut commands, &mut meshes, &mut materials);
}

fn setup_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Name::new("Camera"),
        Transform::from_xyz(0.0, 0.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_paddle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        paddle::Paddle,
        Name::new("Paddle"),
        paddle::PaddleSize {
            half_width: 4.0,
            half_height: 2.0,
            contact_depth: 1.0,
        },
        Transform::from_xyz(0.0, 0.0, 25.0),
        GlobalTransform::default(),
        Mesh3d(meshes.add(Cuboid::new(8.0, 4.0, 0.5))),
        MeshMaterial3d(materials.add(Color::srgba_u8(124, 144, 255, 150))),
    ));
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        ball::Ball,
        Name::new("Ball"),
        physics::Velocity(Vec3::new(0.0, 0.0, 15.0)),
        physics::Curve::default(),
        Transform::default(),
        GlobalTransform::default(),
        Mesh3d(meshes.add(Sphere::new(ball::BALL_RADIUS))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 200, 0))),
    ));
}

fn setup_lighting(commands: &mut Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
