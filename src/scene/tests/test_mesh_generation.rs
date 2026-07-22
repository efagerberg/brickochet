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
        test_utils::assertions::assert_floats_eq(actual, expected);
    }
}
