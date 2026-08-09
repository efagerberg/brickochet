use bevy::{core_pipeline, post_process, prelude::*};

use crate::{audio, gameplay, health, physics, states};

pub mod mesh_generation;

#[cfg(test)]
mod tests;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(states::GameState::Gameplay),
        setup.before(gameplay::GameplaySet::Initialize),
    );
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let playfield_half_size = Vec3::new(10.0, 5.0, 20.0);

    let paddle = spawn_paddle(
        &mut commands,
        &mut meshes,
        &mut materials,
        playfield_half_size,
    );
    let players = vec![paddle];
    spawn_playfield(
        &mut commands,
        &mut meshes,
        &mut materials,
        &players,
        playfield_half_size,
    );
    setup_camera(&mut commands, playfield_half_size);
    setup_lighting(&mut commands);
    spawn_ball(&mut commands, &mut meshes, &mut materials, asset_server);
}

fn spawn_playfield(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    players: &[Entity],
    half_size: Vec3,
) -> gameplay::playfield::resources::Playfield {
    let wall_material = materials.add(Color::srgb(0.0, 0.0, 0.0));
    let clear_wall_material = materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0));

    let line_thickness = 0.25;

    let mut children = vec![];

    let line_highlight_color = LinearRgba::rgb(0.0, 0.4, 0.2);

    let line_material = materials.add(StandardMaterial {
        emissive: line_highlight_color,
        ..default()
    });
    let mesh = mesh_generation::generate(mesh_generation::outline_geometry(
        half_size.truncate(),
        line_thickness,
    ));
    let mesh = meshes.add(mesh);

    children.push(
        commands
            .spawn((
                gameplay::playfield::components::DepthLine,
                Name::new("Depth Line"),
                Mesh3d(mesh),
                MeshMaterial3d(line_material.clone()),
                Transform::from_xyz(0.0, 0.0, half_size.z),
            ))
            .id(),
    );

    spawn_playfield_walls(
        commands,
        meshes,
        (wall_material.clone(), clear_wall_material.clone()),
        &mut children,
        players,
        half_size,
        0.1,
    );

    let parent_entity = commands
        .spawn((
            Name::new("Playfield"),
            Transform::default(),
            GlobalTransform::default(),
            DespawnOnExit(states::GameState::Gameplay),
        ))
        .id();
    for &child in &children {
        commands.entity(parent_entity).add_child(child);
    }

    let playfield = gameplay::playfield::resources::Playfield {
        ball_distance_near_color: LinearRgba::new(0.25, 0.0, 0.0, 1.0),
        ball_distance_far_color: LinearRgba::new(0.0, 0.125, 0.125, 1.0),
        brick_size: Vec2::new(4.0, 2.0),
    };
    commands.insert_resource(playfield.clone());
    playfield
}

fn spawn_playfield_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    wall_materials: (Handle<StandardMaterial>, Handle<StandardMaterial>),
    children: &mut Vec<Entity>,
    players: &[Entity],
    playfield_half_size: Vec3,
    wall_thickness: f32,
) {
    let (solid_wall_material, clear_wall_material) = wall_materials;
    // axis 0 = X, 1 = Y, 2 = Z
    for (axis, size) in [
        // X walls (left/right)
        (
            0,
            Vec3::new(
                wall_thickness,
                2.0 * playfield_half_size.y,
                2.0 * playfield_half_size.z,
            ),
        ),
        // Y walls (floor/ceiling)
        (
            1,
            Vec3::new(
                2.0 * playfield_half_size.x,
                wall_thickness,
                2.0 * playfield_half_size.z,
            ),
        ),
        // Z walls (back/front)
        (
            2,
            Vec3::new(
                2.0 * playfield_half_size.x,
                2.0 * playfield_half_size.y,
                wall_thickness,
            ),
        ),
    ] {
        // For each side: -1 or +1
        for &side in &[-1.0, 1.0] {
            let translation = match axis {
                0 => Vec3::new(side * playfield_half_size.x, 0.0, 0.0),
                1 => Vec3::new(0.0, side * playfield_half_size.y, 0.0),
                2 => Vec3::new(0.0, 0.0, side * playfield_half_size.z),
                _ => Vec3::ZERO,
            };

            let name = match (axis, side) {
                (0, -1.0) => "Left",
                (0, 1.0) => "Right",
                (1, -1.0) => "Floor",
                (1, 1.0) => "Ceiling",
                (2, -1.0) => "Near",
                (2, 1.0) => "Far",
                _ => "Wall",
            };
            let (goal, change_on_collision) = match (axis, side) {
                (2, -1.0) => (Some(gameplay::playfield::components::Goal::Enemy), None),
                (2, 1.0) => (
                    Some(gameplay::playfield::components::Goal::Player),
                    Some(health::components::ChangeOnCollision {
                        delta: -1,
                        affected: health::components::Affects::Others(players.to_vec()),
                    }),
                ),
                _ => (None, None),
            };

            let material = if (axis, side) == (2, 1.0) {
                &clear_wall_material
            } else {
                &solid_wall_material
            };

            let wall_entity = commands
                .spawn((
                    Name::new(name),
                    Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(translation),
                    physics::components::CuboidCollider {
                        half_extents: size * 0.5,
                    },
                    physics::components::StaticBody,
                ))
                .id();

            if let Some(goal) = goal {
                commands.entity(wall_entity).insert(goal);
            }
            if let Some(coll) = change_on_collision {
                commands.entity(wall_entity).insert(coll);
            }
            children.push(wall_entity);
        }
    }
}

fn setup_camera(commands: &mut Commands, playfield_half_size: Vec3) {
    commands.spawn((
        Camera3d::default(),
        Name::new("Camera"),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        post_process::bloom::Bloom {
            intensity: 0.05,
            ..default()
        },
        core_pipeline::tonemapping::DebandDither::Enabled,
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_3, // ~60°
            near: 0.1,
            far: 200.0,
            ..default()
        }),
        Transform::from_xyz(0.0, 0.0, playfield_half_size.z + 9.0)
            .looking_at(Vec3::new(0.0, 0.0, -playfield_half_size.z), Vec3::Y),
        bevy_inspector_egui::bevy_egui::PrimaryEguiContext,
        DespawnOnExit(states::GameState::Gameplay),
    ));
}

fn spawn_paddle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    playfield_half_size: Vec3,
) -> Entity {
    let paddle_half_size = Vec2::new(1.5, 1.0);
    let collider = physics::components::PlaneCollider {
        half_extents: paddle_half_size,
        normal: -Vec3::Z,
    };
    let healthy_color = LinearRgba::rgb(0.0, 0.2, 0.1);
    let critical_color = LinearRgba::rgb(0.5, 0.2, 0.1);
    let paddle_decal_thickness = 0.05;
    let paddle_decal_geometry = mesh_generation::Geometry::default()
        .merge(
            &mesh_generation::outline_geometry(paddle_half_size, paddle_decal_thickness),
            Mat4::IDENTITY,
        )
        .merge(
            &mesh_generation::reticle_geometry(paddle_half_size, paddle_decal_thickness, 0.5),
            Mat4::IDENTITY,
        )
        .merge(
            &mesh_generation::outline_geometry(Vec2::new(0.25, 0.25), paddle_decal_thickness),
            Mat4::IDENTITY,
        );

    let reticle_mesh = mesh_generation::generate(paddle_decal_geometry);
    commands
        .spawn((
            gameplay::paddle::components::Paddle,
            Name::new("Player Paddle"),
            collider.clone(),
            (
                physics::components::Velocity(Vec3::ZERO),
                physics::components::KinematicBody,
            ),
            gameplay::paddle::components::PaddleMotionRecord::default(),
            gameplay::paddle::components::PaddleImpactModifiers::starting(),
            Transform::from_xyz(0.0, 0.0, playfield_half_size.z - 2.0),
            GlobalTransform::default(),
            Mesh3d(meshes.add(reticle_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: healthy_color,
                ..default()
            })),
            health::components::HealthColors {
                max: healthy_color,
                min: critical_color,
                color_type: health::components::HealthColorType::Emissive,
            },
            gameplay::player::components::Player {},
            health::components::Health { max: 3, current: 3 },
            DespawnOnExit(states::GameState::Gameplay),
            children![(
                Mesh3d(meshes.add(Plane3d::new(
                    // Point towards camera view so player can see it
                    -collider.normal,
                    collider.half_extents,
                ))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::from(LinearRgba::new(0.0, 0.0, 0.0, 0.85)),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
            )],
        ))
        .id()
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let ball_modifiers = gameplay::ball::components::BallModifiers::starting();
    let ball_texture_handle = asset_server.get_handle("textures/ball.png").unwrap();
    let ball_material = materials.add(StandardMaterial {
        base_color_texture: Some(ball_texture_handle),
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.95), // tint, WHITE means no tint so texture colors show as-is
        emissive: LinearRgba::rgb(0.01, 0.02, 0.01),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.spawn((
        ball_modifiers.clone(),
        Name::new("Ball"),
        physics::components::Curve::default(),
        physics::components::Velocity(ball_modifiers.base_velocity),
        physics::components::SphereCollider {
            radius: ball_modifiers.base_radius,
        },
        physics::components::DynamicBody,
        Transform::default(),
        GlobalTransform::default(),
        Mesh3d(meshes.add(Sphere::new(ball_modifiers.base_radius))),
        MeshMaterial3d(ball_material),
        DespawnOnExit(states::GameState::Gameplay),
        audio::components::CollisionSFX {
            handle: asset_server
                .get_handle::<AudioSource>("audio/tennisBallHit.ogg")
                .unwrap(),
            volume: 1.0,
        },
    ));
}

fn setup_lighting(commands: &mut Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(-0.7)),
        DespawnOnExit(states::GameState::Gameplay),
    ));
}
