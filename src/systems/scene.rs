use bevy::{asset, mesh, prelude::*};

use crate::{components, resources};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let playfield = spawn_playfield(&mut commands, &mut meshes, &mut materials);
    setup_camera(&mut commands, &playfield);
    setup_lighting(&mut commands);
    spawn_paddle(&mut commands, &mut meshes, &mut materials, &playfield);
    spawn_ball(&mut commands, &mut meshes, &mut materials);
}

fn setup_camera(commands: &mut Commands, playfield: &resources::playfield::Playfield) {
    commands.spawn((
        Camera3d::default(),
        Name::new("Camera"),
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_3, // ~60°
            near: 0.1,
            far: 200.0,
            ..default()
        }),
        Transform::from_xyz(0.0, 0.0, playfield.half_depth + 9.0)
            .looking_at(Vec3::new(0.0, 0.0, -playfield.half_depth), Vec3::Y),
    ));
}

fn spawn_paddle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    playfield: &resources::playfield::Playfield,
) {
    let paddle_size = components::paddle::PaddleSize {
        half_width: 2.0,
        half_height: 1.0,
        contact_depth: 0.1,
    };
    commands.spawn((
        components::paddle::Paddle,
        Name::new("Paddle"),
        paddle_size,
        components::paddle::PaddleMotionRecord::default(),
        Transform::from_xyz(0.0, 0.0, playfield.half_depth - 4.0),
        GlobalTransform::default(),
        Mesh3d(meshes.add(Cuboid::new(
            paddle_size.half_width * 2.0,
            paddle_size.half_height * 2.0,
            paddle_size.contact_depth * 2.0,
        ))),
        MeshMaterial3d(materials.add(Color::srgba_u8(124, 144, 255, 150))),
    ));
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        components::ball::Ball,
        Name::new("Ball"),
        components::physics::Velocity(Vec3::new(0.0, 0.0, 15.0)),
        components::physics::Curve::default(),
        Transform::default(),
        GlobalTransform::default(),
        Mesh3d(meshes.add(Sphere::new(components::ball::RADIUS))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 200, 0))),
    ));
}

fn spawn_playfield(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> resources::playfield::Playfield {
    let half_width = 10.0;
    let half_height = 5.0;
    let half_depth = 20.0;

    let wall_material = materials.add(Color::srgb(0.0, 0.0, 0.0));

    // Number of depth lines
    let num_lines = 10;
    let line_thickness = 0.25;
    let line_spacing = (half_depth * 2.0) / (num_lines as f32);

    // Collect all children entities
    let mut children = vec![];

    // Base line material
    let line_default_color = Color::linear_rgb(0.0, 0.7, 0.0);
    let line_highlight_color = Color::linear_rgb(0.0, 1.0, 0.4);

    for i in 0..num_lines {
        let z = -half_depth + i as f32 * line_spacing;
        let line_material = materials.add(line_default_color);
        let mesh = meshes.add(build_depth_lines_mesh(
            half_width,
            half_height,
            line_thickness,
        ));

        children.push(
            commands
                .spawn((
                    components::playfield::DepthLines,
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
        wall_material.clone(),
        &mut children,
        half_width,
        half_height,
        half_depth,
        0.1,
    );

    // Spawn parent playfield entity
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

    // Insert resource
    let playfield = resources::playfield::Playfield {
        half_width,
        half_height,
        half_depth,
        wall_line_default_color: line_default_color,
        wall_line_highlight_color: line_highlight_color,
    };
    commands.insert_resource(playfield.clone());
    playfield
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

fn build_depth_lines_mesh(half_width: f32, half_height: f32, line_thickness: f32) -> Mesh {
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
        (half_width * 2.0, Vec3::Y, Mat4::IDENTITY),
        // Lines running along Y (left & right walls)
        (
            half_height * 2.0,
            Vec3::X,
            Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2),
        ),
    ] {
        // Two sides per orientation
        for side in [-1.0, 1.0] {
            let offset = match offset_axis {
                Vec3::Y => Vec3::Y * side * half_height,
                Vec3::X => Vec3::X * side * half_width,
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
    wall_material: Handle<StandardMaterial>,
    children: &mut Vec<Entity>,
    half_width: f32,
    half_height: f32,
    half_depth: f32,
    wall_thickness: f32,
) {
    // For each axis: 0=X, 1=Y, 2=Z
    for (axis, size) in [
        (
            0,
            Vec3::new(2.0 * half_width, 2.0 * half_height, wall_thickness),
        ), // Z walls
        (
            1,
            Vec3::new(2.0 * half_width, wall_thickness, 2.0 * half_depth),
        ), // Y walls
        (
            2,
            Vec3::new(wall_thickness, 2.0 * half_height, 2.0 * half_depth),
        ), // X walls
    ] {
        // For each side: -1 or +1
        for &side in &[-1.0, 1.0] {
            // Skip front wall (Z axis, side +1)
            if axis == 0 && side > 0.0 {
                continue;
            }

            let translation = match axis {
                0 => Vec3::new(0.0, 0.0, side * half_depth), // back/front
                1 => Vec3::new(0.0, side * half_height, 0.0), // floor/ceiling
                2 => Vec3::new(side * half_width, 0.0, 0.0), // left/right
                _ => Vec3::ZERO,
            };

            let name = match (axis, side) {
                (0, -1.0) => "Back Wall",
                (1, -1.0) => "Floor",
                (1, 1.0) => "Ceiling",
                (2, -1.0) => "Left Wall",
                (2, 1.0) => "Right Wall",
                _ => "Wall",
            };

            children.push(
                commands
                    .spawn((
                        Name::new(name),
                        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_translation(translation),
                    ))
                    .id(),
            );
        }
    }
}
