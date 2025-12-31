use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

mod components;
mod resources;
mod systems;

use components::{ball::*, paddle::*};
use resources::playfield::Playfield;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default())
        .insert_resource(Playfield {
            width: 10.0,
            height: 10.0,
            depth: 10.0,
        })
        .add_systems(
            Startup,
            (setup_camera, setup_playfield, spawn_paddle, spawn_ball),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Name::new("Camera"),
        Transform::from_xyz(0.0, 0.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_playfield(_commands: Commands) {
    // Example: spawn 4 walls (left, right, top, bottom)
}

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Paddle,
        Name::new("Paddle"),
        PaddleSpeed(8.0),
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
        Ball,
        Name::new("Ball"),
        Velocity(Vec3::new(2.0, 2.0, 0.0)),
        Transform::default(),
        GlobalTransform::default(),
        RigidBody::KinematicPositionBased,
        Collider::ball(4.0),
        Mesh3d(meshes.add(Sphere::new(4.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 200, 0))),
    ));
}
