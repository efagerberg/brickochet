use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{ball, paddle};

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
        Transform::from_xyz(0.0, 0.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        paddle::PaddleSpeed(8.0),
        Transform::from_xyz(0.0, 0.0, 5.0),
        GlobalTransform::default(),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(4.0, 2.0, 0.25),
        Restitution::coefficient(1.0),
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
        ball::Velocity(Vec3::new(0.0, 0.0, 0.0)),
        Transform::default(),
        GlobalTransform::default(),
        RigidBody::KinematicPositionBased,
        Collider::ball(2.0),
        Mesh3d(meshes.add(Sphere::new(2.0))),
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