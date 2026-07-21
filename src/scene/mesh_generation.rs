use bevy::{asset, mesh, prelude::*};

/// Appends `source`'s geometry into `positions`/`indices`, transformed into the target space.
fn append_mesh(
    positions: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    source: &Mesh,
    transform: Mat4,
) {
    let vertices = source
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|attr| attr.as_float3())
        .expect("source mesh must have Position as Float32x3");

    let index_offset = positions.len() as u32;
    positions.extend(vertices.iter().map(|v| {
        let p: Vec3 = transform.transform_point3((*v).into());
        p.to_array()
    }));

    match source.indices() {
        Some(mesh::Indices::U32(src)) => {
            indices.extend(src.iter().map(|i| i + index_offset));
        }
        Some(mesh::Indices::U16(src)) => {
            indices.extend(src.iter().map(|&i| i as u32 + index_offset));
        }
        None => panic!("source mesh must be indexed"),
    }
}

/// Combines several (mesh, transform) parts into a single mesh.
pub fn combine_meshes(parts: impl IntoIterator<Item = (Mesh, Mat4)>) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (part, transform) in parts {
        append_mesh(&mut positions, &mut indices, &part, transform);
    }

    let mut mesh = Mesh::new(
        mesh::PrimitiveTopology::TriangleList,
        asset::RenderAssetUsages::MAIN_WORLD | asset::RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(mesh::Indices::U32(indices));
    mesh.compute_normals();
    mesh
}

/// A single bar along local X, centered at the origin. Transform is applied by the caller.
pub fn generate_line(length: f32, thickness: f32) -> Mesh {
    Mesh::from(Cuboid::new(length, thickness, thickness))
}

/// Two lines crossing through the center, each split into two segments
/// with a gap around the center point.
pub fn generate_cross(half_size: Vec2, thickness: f32, center_gap: f32) -> Mesh {
    let half_gap = center_gap * 0.5;

    let x_len = (half_size.x - half_gap).max(0.0);
    let y_len = (half_size.y - half_gap).max(0.0);
    let x_offset = half_gap + x_len * 0.5;
    let y_offset = half_gap + y_len * 0.5;

    combine_meshes([
        // X axis: left and right segments
        (
            generate_line(x_len, thickness),
            Mat4::from_translation(Vec3::X * x_offset),
        ),
        (
            generate_line(x_len, thickness),
            Mat4::from_translation(Vec3::X * -x_offset),
        ),
        // Y axis: top and bottom segments
        (
            generate_line(y_len, thickness),
            Mat4::from_translation(Vec3::Y * y_offset)
                * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2),
        ),
        (
            generate_line(y_len, thickness),
            Mat4::from_translation(Vec3::Y * -y_offset)
                * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2),
        ),
    ])
}

/// Four lines forming a rectangle border.
pub fn generate_outline(half_size: Vec2, line_thickness: f32) -> Mesh {
    let mut parts = Vec::new();

    for side in [-1.0, 1.0] {
        parts.push((
            generate_line(half_size.x * 2.0, line_thickness),
            Mat4::from_translation(Vec3::Y * side * half_size.y),
        ));
        parts.push((
            generate_line(half_size.y * 2.0, line_thickness),
            Mat4::from_translation(Vec3::X * side * half_size.x)
                * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2),
        ));
    }

    combine_meshes(parts)
}
