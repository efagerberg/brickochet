use bevy::prelude::*;

use crate::{scene::mesh_generation, test_utils};

use test_case::test_case;

#[derive(Default)]
struct GeometryMergeCase {
    seed: mesh_generation::Geometry,
    to_merge: Vec<(mesh_generation::Geometry, Mat4)>,
    expected: mesh_generation::Geometry,
}

#[test_case(
    GeometryMergeCase {
        seed: mesh_generation::Geometry::default(),
        to_merge: vec![(mesh_generation::Geometry::default(), Mat4::IDENTITY)],
        expected: mesh_generation::Geometry::default()
    };
    "e ∘ e = e"
)]
#[test_case(
    GeometryMergeCase {
        seed: mesh_generation::Geometry::default(),
        to_merge: vec![(mesh_generation::Geometry {
            indices: vec![0],
            positions: vec![[0.0, 1.0, 2.0]],
        }, Mat4::IDENTITY)],
        expected: mesh_generation::Geometry {
            indices: vec![0],
            positions: vec![[0.0, 1.0, 2.0]],
        },
    };
    "e ∘ x = x"
)]
#[test_case(
    GeometryMergeCase {
        seed: mesh_generation::Geometry {
            indices: vec![0],
            positions: vec![[0.0, 1.0, 2.0]],
        },
        to_merge: vec![(mesh_generation::Geometry {
            indices: vec![0],
            positions: vec![[3.0, 4.0, 5.0]],
        }, Mat4::IDENTITY)],
        expected: mesh_generation::Geometry {
            indices: vec![0, 1],
            positions: vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]],
        },
    };
    "merging two distinct geometries"
)]
#[test_case(
    GeometryMergeCase {
        seed: mesh_generation::Geometry {
            indices: vec![0],
            positions: vec![[0.0, 1.0, 2.0]],
        },
        to_merge: vec![(mesh_generation::Geometry {
            indices: vec![0],
            positions: vec![[3.0, 4.0, 5.0]],
        }, Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2))],
        expected: mesh_generation::Geometry {
            indices: vec![0, 1],
            positions: vec![[0.0, 1.0, 2.0], [3.0, -5.0, 4.0]],
        },
    };
    "merging two distinct geometries with quarter rotation"
)]
fn test_merge_geometry(case: GeometryMergeCase) {
    let mut merged = case.seed;
    for (geometry, transform) in case.to_merge {
        merged = merged.merge(&geometry, transform);
    }

    assert_eq!(merged.indices, case.expected.indices);
    for (actual, expected) in merged.positions.into_iter().zip(case.expected.positions) {
        test_utils::assertions::assert_floats_approx_eq(actual, expected);
    }
}

struct LineGeometryBoundsCase {
    length: f32,
    thickness: f32,
}

#[test_case(LineGeometryBoundsCase { length: 2.0, thickness: 0.1 }; "long thin line")]
#[test_case(LineGeometryBoundsCase { length: 0.5, thickness: 0.5 }; "cube-like line")]
fn test_line_geometry_positions_within_collider(case: LineGeometryBoundsCase) {
    let geometry = mesh_generation::line_geometry(case.length, case.thickness);
    let half_length = case.length * 0.5;
    let half_thickness = case.thickness * 0.5;

    for [x, y, z] in &geometry.positions {
        assert!(
            x.abs() <= half_length + f32::EPSILON,
            "x {x} exceeds half_length {half_length}"
        );
        assert!(
            y.abs() <= half_thickness + f32::EPSILON,
            "y {y} exceeds half_thickness {half_thickness}"
        );
        assert!(
            z.abs() <= half_thickness + f32::EPSILON,
            "z {z} exceeds half_thickness {half_thickness}"
        );
    }
}

#[test]
fn test_line_geometry_indices_are_valid_triangles() {
    let geometry = mesh_generation::line_geometry(2.0, 0.1);

    assert_eq!(
        geometry.indices.len() % 3,
        0,
        "indices should form complete triangles"
    );
    let vertex_count = geometry.positions.len() as u32;
    assert!(
        geometry.indices.iter().all(|&i| i < vertex_count),
        "all indices must reference a valid vertex"
    );
}

struct ReticleVertexCountCase {
    half_size: Vec2,
    thickness: f32,
    center_gap: f32,
}

#[test_case(
    ReticleVertexCountCase { half_size: Vec2::new(1.5, 1.0), thickness: 0.1, center_gap: 0.5 };
    "gap smaller than half_size on both axes"
)]
#[test_case(
    ReticleVertexCountCase { half_size: Vec2::new(1.0, 1.0), thickness: 0.1, center_gap: 5.0 };
    "gap larger than half_size clamps segment length to zero"
)]
fn test_reticle_geometry_is_four_lines_combined(case: ReticleVertexCountCase) {
    let half_gap = case.center_gap * 0.5;
    let x_len = (case.half_size.x - half_gap).max(0.0);
    let y_len = (case.half_size.y - half_gap).max(0.0);

    let x_line = mesh_generation::line_geometry(x_len, case.thickness);
    let y_line = mesh_generation::line_geometry(y_len, case.thickness);
    let reticle =
        mesh_generation::reticle_geometry(case.half_size, case.thickness, case.center_gap);

    let expected_vertex_count = 2 * x_line.positions.len() + 2 * y_line.positions.len();
    let expected_index_count = 2 * x_line.indices.len() + 2 * y_line.indices.len();

    assert_eq!(reticle.positions.len(), expected_vertex_count);
    assert_eq!(reticle.indices.len(), expected_index_count);
}

#[test]
fn test_outline_geometry_is_four_lines_combined() {
    let half_size = Vec2::new(1.5, 1.0);
    let thickness = 0.1;

    let x_line = mesh_generation::line_geometry(half_size.x * 2.0, thickness);
    let y_line = mesh_generation::line_geometry(half_size.y * 2.0, thickness);
    let outline = mesh_generation::outline_geometry(half_size, thickness);

    let expected_vertex_count = 2 * x_line.positions.len() + 2 * y_line.positions.len();
    let expected_index_count = 2 * x_line.indices.len() + 2 * y_line.indices.len();

    assert_eq!(outline.positions.len(), expected_vertex_count);
    assert_eq!(outline.indices.len(), expected_index_count);
}

#[test]
fn test_outline_geometry_bounding_extent_matches_half_size() {
    let half_size = Vec2::new(1.5, 1.0);
    let thickness = 0.1;
    let outline = mesh_generation::outline_geometry(half_size, thickness);

    let max_x = outline
        .positions
        .iter()
        .map(|p| p[0].abs())
        .fold(0.0, f32::max);
    let max_y = outline
        .positions
        .iter()
        .map(|p| p[1].abs())
        .fold(0.0, f32::max);

    // outer edge of the border sits at half_size + half thickness
    let expected_max_x = half_size.x + thickness * 0.5;
    let expected_max_y = half_size.y + thickness * 0.5;

    test_utils::assertions::assert_floats_approx_eq([max_x], [expected_max_x]);
    test_utils::assertions::assert_floats_approx_eq([max_y], [expected_max_y]);
}

#[test]
fn test_generate_round_trips_positions_and_indices() {
    let geometry = mesh_generation::line_geometry(2.0, 0.1);
    let expected_positions = geometry.positions.clone();
    let expected_indices = geometry.indices.clone();

    let mesh = mesh_generation::generate(geometry);

    let actual_positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|a| a.as_float3())
        .expect("mesh should have position attribute");
    assert_eq!(actual_positions, expected_positions.as_slice());

    let bevy::mesh::Indices::U32(actual_indices) =
        mesh.indices().expect("mesh should have indices")
    else {
        panic!("expected U32 indices");
    };
    assert_eq!(actual_indices, &expected_indices);
}

#[test]
fn test_generate_computes_normals() {
    let geometry = mesh_generation::line_geometry(2.0, 0.1);
    let mesh = mesh_generation::generate(geometry);

    assert!(
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some(),
        "generate should populate normals"
    );
}
