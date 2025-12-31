use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui, quick};
use bevy_rapier3d::prelude::*;

mod components;
mod resources;
mod systems;

use components::{ball, paddle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(quick::WorldInspectorPlugin::default())
        .add_systems(
            Startup,
            (setup_camera, setup_lighting, spawn_paddle, spawn_ball),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Name::new("Camera"),
        Transform::from_xyz(0.0, 0.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        paddle::Paddle,
        Name::new("Paddle"),
        paddle::PaddleSpeed(8.0),
        Transform::default(),
        GlobalTransform::default(),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(1.0, 2.0, 0.5),
        Restitution::coefficient(1.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    ));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        ball::Ball,
        Name::new("Ball"),
        ball::Velocity(Vec3::new(2.0, 2.0, 0.0)),
        Transform::default(),
        GlobalTransform::default(),
        RigidBody::KinematicPositionBased,
        Collider::ball(4.0),
        Mesh3d(meshes.add(Sphere::new(4.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 200, 0))),
    ));
}

fn setup_lighting(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
