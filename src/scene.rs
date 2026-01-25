use bevy::{asset, core_pipeline, mesh, post_process, prelude::*};

use crate::{ball, paddle, physics, playfield};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let playfield_half_size = Vec3::new(10.0, 5.0, 20.0);
    spawn_playfield(
        &mut commands,
        &mut meshes,
        &mut materials,
        playfield_half_size,
    );
    setup_camera(&mut commands, playfield_half_size);
    setup_lighting(&mut commands);
    spawn_paddle(
        &mut commands,
        &mut meshes,
        &mut materials,
        playfield_half_size,
    );
    spawn_ball(&mut commands, &mut meshes, &mut materials);
}

fn spawn_playfield(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    half_size: Vec3,
) -> playfield::resources::Playfield {
    let wall_material = materials.add(Color::srgb(0.0, 0.0, 0.0));
    let clear_wall_material = materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0));

    let num_lines = 10;
    let line_thickness = 0.25;
    let line_spacing = (half_size.z * 2.0) / (num_lines as f32);

    let mut children = vec![];

    let line_default_color = LinearRgba::rgb(0.0, 0.15, 0.0);
    let line_highlight_color = LinearRgba::rgb(0.0, 0.4, 0.2);

    for i in 0..num_lines {
        let z = -half_size.z + i as f32 * line_spacing;
        let line_material = materials.add(StandardMaterial {
            emissive: line_default_color,
            ..default()
        });
        let mesh = meshes.add(build_depth_lines_mesh(half_size, line_thickness));

        children.push(
            commands
                .spawn((
                    playfield::components::DepthLines,
                    Name::new(format!("Depth Line {}", i)),
                    Mesh3d(mesh),
                    MeshMaterial3d(line_material.clone()),
                    Transform::from_xyz(0.0, 0.0, z),
                ))
                .id(),
        );
    }

    spawn_playfield_walls(
        commands,
        meshes,
        (wall_material.clone(), clear_wall_material.clone()),
        &mut children,
        half_size,
        0.1,
    );

    let parent_entity = commands
        .spawn((
            Name::new("Playfield"),
            Transform::default(),
            GlobalTransform::default(),
        ))
        .id();
    for &child in &children {
        commands.entity(parent_entity).add_child(child);
    }

    let playfield = playfield::resources::Playfield {
        wall_line_default_color: line_default_color,
        wall_line_highlight_color: line_highlight_color,
    };
    commands.insert_resource(playfield.clone());
    playfield
}

fn build_depth_lines_mesh(half_size: Vec3, line_thickness: f32) -> Mesh {
    let mut mesh = Mesh::new(
        mesh::PrimitiveTopology::TriangleList,
        asset::RenderAssetUsages::MAIN_WORLD | asset::RenderAssetUsages::RENDER_WORLD,
    );

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut index_offset = 0u32;

    let mut add_cuboid = |size: Vec3, transform: Mat4| {
        let cuboid = Cuboid::new(size.x, size.y, size.z);
        let temp_mesh = Mesh::from(cuboid);

        let vertices = temp_mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|attr| attr.as_float3())
            .expect("Cuboid mesh must have Position as Float32x3");

        for v in vertices {
            let v = transform.transform_point3((*v).into());
            positions.push(v.into());
        }

        if let Some(mesh::Indices::U32(src)) = temp_mesh.indices() {
            indices.extend(src.iter().map(|i| i + index_offset));
            index_offset += vertices.len() as u32;
        }
    };

    for (length, offset_axis, rotation) in [
        // Lines running along X (floor & ceiling)
        (half_size.x * 2.0, Vec3::Y, Mat4::IDENTITY),
        // Lines running along Y (left & right walls)
        (
            half_size.y * 2.0,
            Vec3::X,
            Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2),
        ),
    ] {
        // Two sides per orientation
        for side in [-1.0, 1.0] {
            let offset = match offset_axis {
                Vec3::Y => Vec3::Y * side * half_size.y,
                Vec3::X => Vec3::X * side * half_size.x,
                _ => Vec3::ZERO,
            };

            add_cuboid(
                Vec3::new(length, line_thickness, line_thickness),
                Mat4::from_translation(offset) * rotation,
            );
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(mesh::Indices::U32(indices));

    mesh
}

fn spawn_playfield_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    wall_materials: (Handle<StandardMaterial>, Handle<StandardMaterial>),
    children: &mut Vec<Entity>,
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
            let goal = match (axis, side) {
                (2, -1.0) => Some(playfield::components::Goal::Enemy),
                (2, 1.0) => Some(playfield::components::Goal::Player),
                _ => None,
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
                    physics::components::BoundingCuboid {
                        half_extents: size / 2.0,
                    },
                ))
                .id();

            if let Some(goal) = goal {
                commands.entity(wall_entity).insert(goal);
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
    ));
}

fn spawn_paddle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    playfield_half_size: Vec3,
) {
    let bounds = physics::components::BoundingCuboid {
        half_extents: Vec3::new(2.0, 1.0, 0.1),
    };
    let cuboid_dimensions = bounds.half_extents * 2.0;
    commands.spawn((
        paddle::components::Paddle,
        Name::new("Paddle"),
        bounds,
        paddle::components::PaddleMotionRecord::default(),
        paddle::components::PaddleImpactModifiers::starting(),
        Transform::from_xyz(0.0, 0.0, playfield_half_size.z - 4.0),
        GlobalTransform::default(),
        Mesh3d(meshes.add(Cuboid::new(
            cuboid_dimensions.x,
            cuboid_dimensions.y,
            cuboid_dimensions.z,
        ))),
        MeshMaterial3d(materials.add(Color::srgba_u8(124, 144, 255, 150))),
    ));
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let ball_modifiers = ball::components::BallModifiers::starting();
    commands.spawn((
        ball_modifiers.clone(),
        Name::new("Ball"),
        physics::components::Curve::default(),
        physics::components::Velocity(ball_modifiers.base_velocity),
        physics::components::BoundingSphere {
            radius: ball_modifiers.base_radius,
        },
        Transform::default(),
        GlobalTransform::default(),
        Mesh3d(meshes.add(Sphere::new(ball_modifiers.base_radius))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 200, 0))),
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
    ));
}
