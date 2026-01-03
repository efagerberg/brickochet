use bevy::prelude::*;

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

    let wall_material = materials.add(Color::srgb(0.2, 0.2, 0.25));

    // Number of depth lines
    let num_lines = 9;
    let line_thickness = 0.25;
    let line_spacing = (half_depth * 2.0) / (num_lines as f32);

    // Collect all children entities
    let mut children = vec![];

    // Base line material
    let line_default_color = Color::linear_rgb(0.3, 0.3, 0.35);
    let line_highlight_color = Color::linear_rgb(0.6, 0.6, 0.65);

    for i in 0..num_lines {
        let z = -half_depth + i as f32 * line_spacing;

        // Floor line (X direction)
        children.push(
            commands
                .spawn((
                    components::playfield::DepthLine,
                    Name::new(format!("Floor Line {}", i)),
                    Mesh3d(meshes.add(Cuboid::new(
                        half_width * 2.0,
                        line_thickness,
                        line_thickness,
                    ))),
                    MeshMaterial3d(materials.add(line_default_color)),
                    Transform::from_xyz(0.0, -half_height, z),
                ))
                .id(),
        );

        // Ceiling line (X direction)
        children.push(
            commands
                .spawn((
                    components::playfield::DepthLine,
                    Name::new(format!("Ceiling Line {}", i)),
                    Mesh3d(meshes.add(Cuboid::new(
                        half_width * 2.0,
                        line_thickness,
                        line_thickness,
                    ))),
                    MeshMaterial3d(materials.add(line_default_color)),
                    Transform::from_xyz(0.0, half_height, z),
                ))
                .id(),
        );

        // Left wall line (Z direction, rotated)
        children.push(
            commands
                .spawn((
                    components::playfield::DepthLine,
                    Name::new(format!("Left Wall Line {}", i)),
                    Mesh3d(meshes.add(Cuboid::new(
                        half_height * 2.0,
                        line_thickness,
                        line_thickness,
                    ))),
                    MeshMaterial3d(materials.add(line_default_color)),
                    Transform {
                        translation: Vec3::new(-half_width, 0.0, z),
                        rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                        ..Default::default()
                    },
                ))
                .id(),
        );

        // Right wall line (Z direction, rotated)
        children.push(
            commands
                .spawn((
                    components::playfield::DepthLine,
                    Name::new(format!("Right Wall Line {}", i)),
                    Mesh3d(meshes.add(Cuboid::new(
                        half_height * 2.0,
                        line_thickness,
                        line_thickness,
                    ))),
                    MeshMaterial3d(materials.add(line_default_color)),
                    Transform {
                        translation: Vec3::new(half_width, 0.0, z),
                        rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                        ..Default::default()
                    },
                ))
                .id(),
        );
    }

    // Static walls
    // Back wall
    children.push(
        commands
            .spawn((
                Name::new("Back Wall"),
                Mesh3d(meshes.add(Cuboid::new(half_width * 2.0, half_height * 2.0, 0.1))),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(0.0, 0.0, -half_depth),
            ))
            .id(),
    );

    // Floor
    children.push(
        commands
            .spawn((
                Name::new("Floor"),
                Mesh3d(meshes.add(Cuboid::new(half_width * 2.0, 0.1, half_depth * 2.0))),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(0.0, -half_height, 0.0),
            ))
            .id(),
    );

    // Ceiling
    children.push(
        commands
            .spawn((
                Name::new("Ceiling"),
                Mesh3d(meshes.add(Cuboid::new(half_width * 2.0, 0.1, half_depth * 2.0))),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(0.0, half_height, 0.0),
            ))
            .id(),
    );

    // Left wall
    children.push(
        commands
            .spawn((
                Name::new("Left Wall"),
                Mesh3d(meshes.add(Cuboid::new(0.1, half_height * 2.0, half_depth * 2.0))),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(-half_width, 0.0, 0.0),
            ))
            .id(),
    );

    // Right wall
    children.push(
        commands
            .spawn((
                Name::new("Right Wall"),
                Mesh3d(meshes.add(Cuboid::new(0.1, half_height * 2.0, half_depth * 2.0))),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(half_width, 0.0, 0.0),
            ))
            .id(),
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
        wall_line_highlight_color: line_highlight_color
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
