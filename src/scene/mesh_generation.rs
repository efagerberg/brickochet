use bevy::{asset, mesh, prelude::*};

/// Raw triangle geometry: positions and indices, no Bevy `Mesh` involved.
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Geometry {
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl Geometry {
    pub fn merge(mut self, other: &Geometry, transform: Mat4) -> Self {
        let index_offset = self.positions.len() as u32;
        self.positions.extend(
            other
                .positions
                .iter()
                .map(|&p| transform.transform_point3(p.into()).to_array()),
        );
        self.indices
            .extend(other.indices.iter().map(|i| i + index_offset));
        self
    }
}

/// Builds a final renderable `Mesh` from raw geometry. The only place this
/// module touches `Mesh` construction — everything else stays pure data.
pub fn generate(geometry: Geometry) -> Mesh {
    let mut mesh = Mesh::new(
        mesh::PrimitiveTopology::TriangleList,
        asset::RenderAssetUsages::MAIN_WORLD | asset::RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, geometry.positions);
    mesh.insert_indices(mesh::Indices::U32(geometry.indices));
    mesh.compute_normals();
    mesh
}

fn geometry_of(source: &Mesh) -> Geometry {
    let positions = source
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|attr| attr.as_float3())
        .expect("source mesh must have Position as Float32x3")
        .to_vec();

    let indices = match source.indices() {
        Some(mesh::Indices::U32(src)) => src.clone(),
        Some(mesh::Indices::U16(src)) => src.iter().map(|&i| i as u32).collect(),
        None => panic!("source mesh must be indexed"),
    };

    Geometry { positions, indices }
}

pub fn line_geometry(length: f32, thickness: f32) -> Geometry {
    geometry_of(&Mesh::from(Cuboid::new(length, thickness, thickness)))
}

pub fn reticle_geometry(half_size: Vec2, thickness: f32, center_gap: f32) -> Geometry {
    let half_gap = center_gap * 0.5;
    let x_len = (half_size.x - half_gap).max(0.0);
    let y_len = (half_size.y - half_gap).max(0.0);
    let x_offset = half_gap + x_len * 0.5;
    let y_offset = half_gap + y_len * 0.5;

    let x_line = line_geometry(x_len, thickness);
    let y_line = line_geometry(y_len, thickness);

    Geometry::default()
        .merge(&x_line, Mat4::from_translation(Vec3::X * x_offset))
        .merge(&x_line, Mat4::from_translation(Vec3::X * -x_offset))
        .merge(
            &y_line,
            Mat4::from_translation(Vec3::Y * y_offset)
                * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2),
        )
        .merge(
            &y_line,
            Mat4::from_translation(Vec3::Y * -y_offset)
                * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2),
        )
}

pub fn outline_geometry(half_size: Vec2, thickness: f32) -> Geometry {
    let x_line = line_geometry(half_size.x * 2.0, thickness);
    let y_line = line_geometry(half_size.y * 2.0, thickness);

    let mut geometry = Geometry::default();
    for side in [-1.0, 1.0] {
        geometry = geometry
            .merge(
                &x_line,
                Mat4::from_translation(Vec3::Y * side * half_size.y),
            )
            .merge(
                &y_line,
                Mat4::from_translation(Vec3::X * side * half_size.x)
                    * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2),
            );
    }
    geometry
}
